//! GGA_X_LSRPBE kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lsrpbe.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lsrpbe_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_kappa = f64x8::splat(param_kappa);
    let param_alpha = f64x8::splat(param_alpha);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_mu * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t41 = f64x8::splat(1.0) / param_kappa;
            let t45 = (simd::exp(-t34 * v_sigma0 * t39 * t41 / f64x8::splat(24.0)));
            let t48 = param_kappa + f64x8::splat(1.0);
            let t49 = param_alpha * t28;
            let t50 = t33 * v_sigma0;
            let t54 = (simd::exp(-t49 * t50 * t39 / f64x8::splat(24.0)));
            let t57 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t45) - t48 * (f64x8::splat(1.0) - t54);
            let t61 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t57));
            let t62 = (v_rho1).simd_le(dens_threshold);
            let t63 = -t16;
            let t65 = ((t14).select(t11, (t10).select(t15, t63 * t7)));
            let t66 = f64x8::splat(1.0) + t65;
            let t67 = (t66).simd_le(zeta_threshold);
            let t68 = (simd::cbrt(t66));
            let t70 = ((t67).select(t22, t68 * t66));
            let t71 = t70 * t26;
            let t72 = v_rho1 * v_rho1;
            let t73 = (simd::cbrt(v_rho1));
            let t74 = t73 * t73;
            let t76 = f64x8::splat(1.0) / t74 / t72;
            let t81 = (simd::exp(-t34 * v_sigma2 * t76 * t41 / f64x8::splat(24.0)));
            let t84 = t33 * v_sigma2;
            let t88 = (simd::exp(-t49 * t84 * t76 / f64x8::splat(24.0)));
            let t91 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t81) - t48 * (f64x8::splat(1.0) - t88);
            let t95 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t91));
            let tzk0 = t61 + t95;
            acc_zk = tzk0;
            let t96 = t6 * t6;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = t16 * t97;
            let t100 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t98)));
            let t103 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t100));
            let t104 = t103 * t26;
            let t108 = t26 * t26;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t25 * t109;
            let t113 = t5 * t110 * t57 / f64x8::splat(8.0);
            let t114 = t35 * v_rho0;
            let t116 = f64x8::splat(1.0) / t37 / t114;
            let t121 = t48 * param_alpha * t28;
            let t126 = t121 * t50 * t116 * t54 / f64x8::splat(9.0) - t34 * v_sigma0 * t116 * t45 / f64x8::splat(9.0);
            let t131 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t104 * t57 - t113 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t126));
            let t132 = t63 * t97;
            let t134 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t132)));
            let t137 = ((t67).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t134));
            let t138 = t137 * t26;
            let t142 = t70 * t109;
            let t145 = t5 * t142 * t91 / f64x8::splat(8.0);
            let t147 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t91 - t145));
            let tvrho0 = t61 + t95 + t6 * (t131 + t147);
            acc_vrho_0 = tvrho0;
            let t151 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t98)));
            let t154 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t151));
            let t155 = t154 * t26;
            let t160 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t155 * t57 - t113));
            let t162 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t132)));
            let t165 = ((t67).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t162));
            let t166 = t165 * t26;
            let t170 = t72 * v_rho1;
            let t172 = f64x8::splat(1.0) / t74 / t170;
            let t180 = t121 * t84 * t172 * t88 / f64x8::splat(9.0) - t34 * v_sigma2 * t172 * t81 / f64x8::splat(9.0);
            let t185 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t91 - t145 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t180));
            let tvrho1 = t61 + t95 + t6 * (t160 + t185);
            acc_vrho_1 = tvrho1;
            let t188 = t33 * t39;
            let t194 = -t121 * t188 * t54 / f64x8::splat(24.0) + t29 * t188 * t45 / f64x8::splat(24.0);
            let t198 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t194));
            let tvsigma0 = t6 * t198;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t199 = t33 * t76;
            let t205 = -t121 * t199 * t88 / f64x8::splat(24.0) + t29 * t199 * t81 / f64x8::splat(24.0);
            let t209 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t205));
            let tvsigma2 = t6 * t209;
            acc_vsigma_2 = tvsigma2;
            let t212 = t23 * t23;
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t100 * t100;
            let t217 = t96 * t6;
            let t218 = f64x8::splat(1.0) / t217;
            let t219 = t16 * t218;
            let t222 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t97 + f64x8::splat(2.0) * t219)));
            let t226 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t213 * t214 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t222));
            let t227 = t226 * t26;
            let t231 = t103 * t109;
            let t233 = t5 * t231 * t57;
            let t239 = f64x8::splat(1.0) / t108 / t6;
            let t240 = t25 * t239;
            let t243 = t5 * t240 * t57 / f64x8::splat(12.0);
            let t245 = t5 * t110 * t126;
            let t247 = t35 * t35;
            let t249 = f64x8::splat(1.0) / t37 / t247;
            let t254 = param_mu * param_mu;
            let t255 = t28 * t28;
            let t258 = f64x8::splat(1.0) / t31 / t30;
            let t259 = t254 * t255 * t258;
            let t260 = v_sigma0 * v_sigma0;
            let t263 = f64x8::splat(1.0) / t36 / t247 / t114;
            let t265 = t41 * t45;
            let t273 = param_alpha * param_alpha;
            let t275 = t48 * t273 * t255;
            let t276 = t258 * t260;
            let t281 = f64x8::splat(11.0) / f64x8::splat(27.0) * t34 * v_sigma0 * t249 * t45 - t259 * t260 * t263 * t265 / f64x8::splat(81.0) - f64x8::splat(11.0) / f64x8::splat(27.0) * t121 * t50 * t249 * t54 + t275 * t276 * t263 * t54 / f64x8::splat(81.0);
            let t286 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t227 * t57 - t233 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t104 * t126 + t243 - t245 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t281));
            let t287 = t68 * t68;
            let t288 = f64x8::splat(1.0) / t287;
            let t289 = t134 * t134;
            let t292 = t63 * t218;
            let t295 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t97 + f64x8::splat(2.0) * t292)));
            let t299 = ((t67).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t288 * t289 + f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t295));
            let t300 = t299 * t26;
            let t304 = t137 * t109;
            let t306 = t5 * t304 * t91;
            let t308 = t70 * t239;
            let t311 = t5 * t308 * t91 / f64x8::splat(12.0);
            let t313 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t300 * t91 - t306 / f64x8::splat(4.0) + t311));
            let tv2rho20 = f64x8::splat(2.0) * t131 + f64x8::splat(2.0) * t147 + t6 * (t286 + t313);
            acc_v2rho2_0 = tv2rho20;
            let t316 = t213 * t151;
            let t320 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t219)));
            let t324 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t316 * t100 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t320));
            let t325 = t324 * t26;
            let t329 = t154 * t109;
            let t331 = t5 * t329 * t57;
            let t339 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t325 * t57 - t331 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t155 * t126 - t233 / f64x8::splat(8.0) + t243 - t245 / f64x8::splat(8.0)));
            let t340 = t288 * t162;
            let t344 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t292)));
            let t348 = ((t67).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t340 * t134 + f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t344));
            let t349 = t348 * t26;
            let t353 = t165 * t109;
            let t355 = t5 * t353 * t91;
            let t362 = t5 * t142 * t180;
            let t365 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t349 * t91 - t355 / f64x8::splat(8.0) - t306 / f64x8::splat(8.0) + t311 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t180 - t362 / f64x8::splat(8.0)));
            let tv2rho21 = t131 + t147 + t160 + t185 + t6 * (t339 + t365);
            acc_v2rho2_1 = tv2rho21;
            let t370 = t151 * t151;
            let t375 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t97 + f64x8::splat(2.0) * t219)));
            let t379 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t213 * t370 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t375));
            let t380 = t379 * t26;
            let t386 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t380 * t57 - t331 / f64x8::splat(4.0) + t243));
            let t387 = t162 * t162;
            let t392 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t97 + f64x8::splat(2.0) * t292)));
            let t396 = ((t67).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t288 * t387 + f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t392));
            let t397 = t396 * t26;
            let t406 = t72 * t72;
            let t408 = f64x8::splat(1.0) / t74 / t406;
            let t413 = v_sigma2 * v_sigma2;
            let t416 = f64x8::splat(1.0) / t73 / t406 / t170;
            let t418 = t41 * t81;
            let t426 = t258 * t413;
            let t431 = f64x8::splat(11.0) / f64x8::splat(27.0) * t34 * v_sigma2 * t408 * t81 - t259 * t413 * t416 * t418 / f64x8::splat(81.0) - f64x8::splat(11.0) / f64x8::splat(27.0) * t121 * t84 * t408 * t88 + t275 * t426 * t416 * t88 / f64x8::splat(81.0);
            let t436 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t397 * t91 - t355 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t166 * t180 + t311 - t362 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t431));
            let tv2rho22 = f64x8::splat(2.0) * t160 + f64x8::splat(2.0) * t185 + t6 * (t386 + t436);
            acc_v2rho2_2 = tv2rho22;
            let t444 = t5 * t110 * t194 / f64x8::splat(8.0);
            let t445 = t33 * t116;
            let t449 = t247 * t35;
            let t451 = f64x8::splat(1.0) / t36 / t449;
            let t459 = t258 * t451;
            let t460 = v_sigma0 * t54;
            let t464 = -t29 * t445 * t45 / f64x8::splat(9.0) + t259 * t451 * v_sigma0 * t265 / f64x8::splat(216.0) + t121 * t445 * t54 / f64x8::splat(9.0) - t275 * t459 * t460 / f64x8::splat(216.0);
            let t469 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t104 * t194 - t444 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t464));
            let tv2rhosigma0 = t6 * t469 + t198;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t476 = t5 * t142 * t205 / f64x8::splat(8.0);
            let t478 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t205 - t476));
            let tv2rhosigma2 = t6 * t478 + t209;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t484 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t155 * t194 - t444));
            let tv2rhosigma3 = t6 * t484 + t198;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t489 = t33 * t172;
            let t493 = t406 * t72;
            let t495 = f64x8::splat(1.0) / t73 / t493;
            let t503 = t258 * t495;
            let t504 = v_sigma2 * t88;
            let t508 = -t29 * t489 * t81 / f64x8::splat(9.0) + t259 * t495 * v_sigma2 * t418 / f64x8::splat(216.0) + t121 * t489 * t88 / f64x8::splat(9.0) - t275 * t503 * t504 / f64x8::splat(216.0);
            let t513 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t205 - t476 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t508));
            let tv2rhosigma5 = t6 * t513 + t209;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t515 = t247 * v_rho0;
            let t517 = f64x8::splat(1.0) / t36 / t515;
            let t525 = t275 * t258 * t517 * t54 / f64x8::splat(576.0) - t259 * t517 * t41 * t45 / f64x8::splat(576.0);
            let t529 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t525));
            let tv2sigma20 = t6 * t529;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t530 = t406 * v_rho1;
            let t532 = f64x8::splat(1.0) / t73 / t530;
            let t540 = t275 * t258 * t532 * t88 / f64x8::splat(576.0) - t259 * t532 * t41 * t81 / f64x8::splat(576.0);
            let t544 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t540));
            let tv2sigma25 = t6 * t544;
            acc_v2sigma2_5 = tv2sigma25;
            let t548 = f64x8::splat(1.0) / t212 / t19;
            let t549 = t214 * t100;
            let t552 = t213 * t100;
            let t555 = t96 * t96;
            let t556 = f64x8::splat(1.0) / t555;
            let t557 = t16 * t556;
            let t560 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t218 - f64x8::splat(6.0) * t557)));
            let t564 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t548 * t549 + f64x8::splat(4.0) / f64x8::splat(3.0) * t552 * t222 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t560));
            let t565 = t564 * t26;
            let t569 = t226 * t109;
            let t571 = t5 * t569 * t57;
            let t576 = t103 * t239;
            let t578 = t5 * t576 * t57;
            let t581 = t5 * t231 * t126;
            let t587 = f64x8::splat(1.0) / t108 / t96;
            let t588 = t25 * t587;
            let t591 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t588 * t57;
            let t593 = t5 * t240 * t126;
            let t596 = t5 * t110 * t281;
            let t599 = f64x8::splat(1.0) / t37 / t515;
            let t604 = t247 * t247;
            let t606 = f64x8::splat(1.0) / t36 / t604;
            let t612 = t30 * t30;
            let t613 = f64x8::splat(1.0) / t612;
            let t614 = t254 * param_mu * t613;
            let t615 = t260 * v_sigma0;
            let t616 = t614 * t615;
            let t617 = t604 * t114;
            let t618 = f64x8::splat(1.0) / t617;
            let t619 = param_kappa * param_kappa;
            let t620 = f64x8::splat(1.0) / t619;
            let t634 = t48 * t273 * param_alpha;
            let t635 = t634 * t613;
            let t640 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t34 * v_sigma0 * t599 * t45 + f64x8::splat(11.0) / f64x8::splat(81.0) * t259 * t260 * t606 * t265 - f64x8::splat(2.0) / f64x8::splat(243.0) * t616 * t618 * t620 * t45 + f64x8::splat(154.0) / f64x8::splat(81.0) * t121 * t50 * t599 * t54 - f64x8::splat(11.0) / f64x8::splat(81.0) * t275 * t276 * t606 * t54 + f64x8::splat(2.0) / f64x8::splat(243.0) * t635 * t615 * t618 * t54;
            let t645 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t565 * t57 - f64x8::splat(3.0) / f64x8::splat(8.0) * t571 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t227 * t126 + t578 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t581 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t104 * t281 - t591 + t593 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t596 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t640));
            let t647 = f64x8::splat(1.0) / t287 / t66;
            let t648 = t289 * t134;
            let t651 = t288 * t134;
            let t654 = t63 * t556;
            let t657 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t218 - f64x8::splat(6.0) * t654)));
            let t661 = ((t67).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t647 * t648 + f64x8::splat(4.0) / f64x8::splat(3.0) * t651 * t295 + f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t657));
            let t662 = t661 * t26;
            let t666 = t299 * t109;
            let t668 = t5 * t666 * t91;
            let t670 = t137 * t239;
            let t672 = t5 * t670 * t91;
            let t674 = t70 * t587;
            let t677 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t674 * t91;
            let t679 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t662 * t91 - f64x8::splat(3.0) / f64x8::splat(8.0) * t668 + t672 / f64x8::splat(4.0) - t677));
            let tv3rho30 = f64x8::splat(3.0) * t286 + f64x8::splat(3.0) * t313 + t6 * (t645 + t679);
            acc_v3rho3_0 = tv3rho30;
            let t682 = f64x8::splat(2.0) * t339;
            let t683 = f64x8::splat(2.0) * t365;
            let t684 = t548 * t151;
            let t687 = t213 * t320;
            let t692 = f64x8::splat(2.0) * t218;
            let t693 = f64x8::splat(6.0) * t557;
            let t695 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t692 - t693)));
            let t699 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t684 * t214 + f64x8::splat(8.0) / f64x8::splat(9.0) * t687 * t100 + f64x8::splat(4.0) / f64x8::splat(9.0) * t316 * t222 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t695));
            let t700 = t699 * t26;
            let t704 = t324 * t109;
            let t707 = t5 * t704 * t57 / f64x8::splat(4.0);
            let t711 = t154 * t239;
            let t713 = t5 * t711 * t57;
            let t717 = t5 * t329 * t126 / f64x8::splat(4.0);
            let t726 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t700 * t57 - t707 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t325 * t126 + t713 / f64x8::splat(12.0) - t717 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t155 * t281 - t571 / f64x8::splat(8.0) + t578 / f64x8::splat(6.0) - t581 / f64x8::splat(4.0) - t591 + t593 / f64x8::splat(6.0) - t596 / f64x8::splat(8.0);
            let t727 = ((t1).select(f64x8::splat(0.0), t726));
            let t728 = t647 * t162;
            let t731 = t288 * t344;
            let t736 = f64x8::splat(6.0) * t654;
            let t738 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t692 - t736)));
            let t742 = ((t67).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t728 * t289 + f64x8::splat(8.0) / f64x8::splat(9.0) * t731 * t134 + f64x8::splat(4.0) / f64x8::splat(9.0) * t340 * t295 + f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t738));
            let t743 = t742 * t26;
            let t747 = t348 * t109;
            let t750 = t5 * t747 * t91 / f64x8::splat(4.0);
            let t751 = t165 * t239;
            let t753 = t5 * t751 * t91;
            let t762 = t5 * t304 * t180 / f64x8::splat(4.0);
            let t764 = t5 * t308 * t180;
            let t767 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t743 * t91 - t750 + t753 / f64x8::splat(12.0) - t668 / f64x8::splat(8.0) + t672 / f64x8::splat(6.0) - t677 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t300 * t180 - t762 + t764 / f64x8::splat(12.0)));
            let tv3rho31 = t286 + t313 + t682 + t683 + t6 * (t727 + t767);
            acc_v3rho3_1 = tv3rho31;
            let t770 = t548 * t370;
            let t775 = t213 * t375;
            let t779 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t692 - t693)));
            let t783 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t770 * t100 + f64x8::splat(8.0) / f64x8::splat(9.0) * t316 * t320 + f64x8::splat(4.0) / f64x8::splat(9.0) * t775 * t100 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t779));
            let t784 = t783 * t26;
            let t788 = t379 * t109;
            let t790 = t5 * t788 * t57;
            let t799 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t784 * t57 - t790 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t380 * t126 - t707 + t713 / f64x8::splat(6.0) - t717 + t578 / f64x8::splat(12.0) - t591 + t593 / f64x8::splat(12.0)));
            let t800 = t647 * t387;
            let t805 = t288 * t392;
            let t809 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t692 - t736)));
            let t813 = ((t67).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t800 * t134 + f64x8::splat(8.0) / f64x8::splat(9.0) * t340 * t344 + f64x8::splat(4.0) / f64x8::splat(9.0) * t805 * t134 + f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t809));
            let t814 = t813 * t26;
            let t818 = t396 * t109;
            let t820 = t5 * t818 * t91;
            let t827 = t5 * t353 * t180;
            let t835 = t5 * t142 * t431;
            let t837 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t814 * t91 - t820 / f64x8::splat(8.0) - t750 + t753 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t349 * t180 - t827 / f64x8::splat(4.0) + t672 / f64x8::splat(12.0) - t677 - t762 + t764 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t431 - t835 / f64x8::splat(8.0);
            let t838 = ((t62).select(f64x8::splat(0.0), t837));
            let tv3rho32 = t682 + t683 + t386 + t436 + t6 * (t799 + t838);
            acc_v3rho3_2 = tv3rho32;
            let t843 = t370 * t151;
            let t850 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t218 - f64x8::splat(6.0) * t557)));
            let t854 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t548 * t843 + f64x8::splat(4.0) / f64x8::splat(3.0) * t316 * t375 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t850));
            let t855 = t854 * t26;
            let t862 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t855 * t57 - f64x8::splat(3.0) / f64x8::splat(8.0) * t790 + t713 / f64x8::splat(4.0) - t591));
            let t863 = t387 * t162;
            let t870 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t218 - f64x8::splat(6.0) * t654)));
            let t874 = ((t67).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t647 * t863 + f64x8::splat(4.0) / f64x8::splat(3.0) * t340 * t392 + f64x8::splat(4.0) / f64x8::splat(3.0) * t68 * t870));
            let t875 = t874 * t26;
            let t891 = f64x8::splat(1.0) / t74 / t530;
            let t896 = t406 * t406;
            let t898 = f64x8::splat(1.0) / t73 / t896;
            let t903 = t413 * v_sigma2;
            let t904 = t614 * t903;
            let t905 = t896 * t170;
            let t906 = f64x8::splat(1.0) / t905;
            let t923 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t34 * v_sigma2 * t891 * t81 + f64x8::splat(11.0) / f64x8::splat(81.0) * t259 * t413 * t898 * t418 - f64x8::splat(2.0) / f64x8::splat(243.0) * t904 * t906 * t620 * t81 + f64x8::splat(154.0) / f64x8::splat(81.0) * t121 * t84 * t891 * t88 - f64x8::splat(11.0) / f64x8::splat(81.0) * t275 * t426 * t898 * t88 + f64x8::splat(2.0) / f64x8::splat(243.0) * t635 * t903 * t906 * t88;
            let t928 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t875 * t91 - f64x8::splat(3.0) / f64x8::splat(8.0) * t820 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t397 * t180 + t753 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t827 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t166 * t431 - t677 + t764 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t835 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t923));
            let tv3rho33 = f64x8::splat(3.0) * t386 + f64x8::splat(3.0) * t436 + t6 * (t862 + t928);
            acc_v3rho3_3 = tv3rho33;
            let t936 = t5 * t231 * t194;
            let t943 = t5 * t240 * t194 / f64x8::splat(12.0);
            let t945 = t5 * t110 * t464;
            let t947 = t33 * t249;
            let t955 = t604 * t35;
            let t956 = f64x8::splat(1.0) / t955;
            let t957 = t614 * t956;
            let t959 = t260 * t620 * t45;
            let t965 = t258 * t263;
            let t973 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t947 * t45 - t259 * t263 * v_sigma0 * t265 / f64x8::splat(24.0) + t957 * t959 / f64x8::splat(324.0) - f64x8::splat(11.0) / f64x8::splat(27.0) * t121 * t947 * t54 + t275 * t965 * t460 / f64x8::splat(24.0) - t635 * t956 * t260 * t54 / f64x8::splat(324.0);
            let t978 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t227 * t194 - t936 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t104 * t464 + t943 - t945 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t973));
            let tv3rho2sigma0 = t6 * t978 + f64x8::splat(2.0) * t469;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t985 = t5 * t304 * t205;
            let t989 = t5 * t308 * t205 / f64x8::splat(12.0);
            let t991 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t300 * t205 - t985 / f64x8::splat(4.0) + t989));
            let tv3rho2sigma2 = t6 * t991 + f64x8::splat(2.0) * t478;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t997 = t5 * t329 * t194;
            let t1005 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t325 * t194 - t997 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t155 * t464 - t936 / f64x8::splat(8.0) + t943 - t945 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1005 + t469 + t484;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1011 = t5 * t353 * t205;
            let t1018 = t5 * t142 * t508;
            let t1021 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t349 * t205 - t1011 / f64x8::splat(8.0) - t985 / f64x8::splat(8.0) + t989 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t508 - t1018 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1021 + t478 + t513;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1029 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t380 * t194 - t997 / f64x8::splat(4.0) + t943));
            let tv3rho2sigma6 = t6 * t1029 + f64x8::splat(2.0) * t484;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1040 = t33 * t408;
            let t1048 = t896 * t72;
            let t1049 = f64x8::splat(1.0) / t1048;
            let t1050 = t614 * t1049;
            let t1052 = t413 * t620 * t81;
            let t1058 = t258 * t416;
            let t1066 = f64x8::splat(11.0) / f64x8::splat(27.0) * t29 * t1040 * t81 - t259 * t416 * v_sigma2 * t418 / f64x8::splat(24.0) + t1050 * t1052 / f64x8::splat(324.0) - f64x8::splat(11.0) / f64x8::splat(27.0) * t121 * t1040 * t88 + t275 * t1058 * t504 / f64x8::splat(24.0) - t635 * t1049 * t413 * t88 / f64x8::splat(324.0);
            let t1071 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t397 * t205 - t1011 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t166 * t508 + t989 - t1018 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t1066));
            let tv3rho2sigma8 = t6 * t1071 + f64x8::splat(2.0) * t513;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1078 = t5 * t110 * t525 / f64x8::splat(8.0);
            let t1083 = t604 * v_rho0;
            let t1084 = f64x8::splat(1.0) / t1083;
            let t1087 = t620 * v_sigma0 * t45;
            let t1097 = t259 * t451 * t41 * t45 / f64x8::splat(108.0) - t614 * t1084 * t1087 / f64x8::splat(864.0) - t275 * t459 * t54 / f64x8::splat(108.0) + t635 * t1084 * v_sigma0 * t54 / f64x8::splat(864.0);
            let t1102 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t104 * t525 - t1078 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1097));
            let tv3rhosigma20 = t6 * t1102 + t529;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1109 = t5 * t142 * t540 / f64x8::splat(8.0);
            let t1111 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t540 - t1109));
            let tv3rhosigma25 = t6 * t1111 + t544;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1117 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t155 * t525 - t1078));
            let tv3rhosigma26 = t6 * t1117 + t529;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1126 = t896 * v_rho1;
            let t1127 = f64x8::splat(1.0) / t1126;
            let t1130 = t620 * v_sigma2 * t81;
            let t1140 = t259 * t495 * t41 * t81 / f64x8::splat(108.0) - t614 * t1127 * t1130 / f64x8::splat(864.0) - t275 * t503 * t88 / f64x8::splat(108.0) + t635 * t1127 * v_sigma2 * t88 / f64x8::splat(864.0);
            let t1145 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t540 - t1109 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t1140));
            let tv3rhosigma211 = t6 * t1145 + t544;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1147 = f64x8::splat(1.0) / t604;
            let t1155 = t614 * t1147 * t620 * t45 / f64x8::splat(2304.0) - t634 * t613 * t1147 * t54 / f64x8::splat(2304.0);
            let t1159 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1155));
            let tv3sigma30 = t6 * t1159;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1160 = f64x8::splat(1.0) / t896;
            let t1168 = -t634 * t613 * t1160 * t88 / f64x8::splat(2304.0) + t614 * t1160 * t620 * t81 / f64x8::splat(2304.0);
            let t1172 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t71 * t1168));
            let tv3sigma39 = t6 * t1172;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
