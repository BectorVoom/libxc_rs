//! GGA_X_PBEINT kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbeint.c`
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
pub fn gga_x_pbeint_kxc_pol(
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
    param_muPBE: f64,
    param_muGE: f64,
    param_alpha: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_muPBE = f64x8::splat(param_muPBE);
    let param_muGE = f64x8::splat(param_muGE);
    let param_alpha = f64x8::splat(param_alpha);
    let param_kappa = f64x8::splat(param_kappa);
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
            let t28 = param_muPBE - param_muGE;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = t28 * param_alpha * t30;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t35 * v_sigma0;
            let t37 = v_rho0 * v_rho0;
            let t38 = (simd::cbrt(v_rho0));
            let t39 = t38 * t38;
            let t41 = f64x8::splat(1.0) / t39 / t37;
            let t42 = param_alpha * t30;
            let t43 = t36 * t41;
            let t46 = f64x8::splat(1.0) + t42 * t43 / f64x8::splat(24.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t53 = (param_muGE + t31 * t36 * t41 * t47 / f64x8::splat(24.0)) * t30;
            let t56 = param_kappa + t53 * t43 / f64x8::splat(24.0);
            let t61 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t56);
            let t65 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t61));
            let t66 = (v_rho1).simd_le(dens_threshold);
            let t67 = -t16;
            let t69 = ((t14).select(t11, (t10).select(t15, t67 * t7)));
            let t70 = f64x8::splat(1.0) + t69;
            let t71 = (t70).simd_le(zeta_threshold);
            let t72 = (simd::cbrt(t70));
            let t74 = ((t71).select(t22, t72 * t70));
            let t75 = t74 * t26;
            let t76 = t35 * v_sigma2;
            let t77 = v_rho1 * v_rho1;
            let t78 = (simd::cbrt(v_rho1));
            let t79 = t78 * t78;
            let t81 = f64x8::splat(1.0) / t79 / t77;
            let t82 = t76 * t81;
            let t85 = f64x8::splat(1.0) + t42 * t82 / f64x8::splat(24.0);
            let t86 = f64x8::splat(1.0) / t85;
            let t92 = (param_muGE + t31 * t76 * t81 * t86 / f64x8::splat(24.0)) * t30;
            let t95 = param_kappa + t92 * t82 / f64x8::splat(24.0);
            let t100 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t95);
            let t104 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t75 * t100));
            let tzk0 = t65 + t104;
            acc_zk = tzk0;
            let t105 = t6 * t6;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t16 * t106;
            let t109 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t107)));
            let t112 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t109));
            let t113 = t112 * t26;
            let t117 = t26 * t26;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t25 * t118;
            let t122 = t5 * t119 * t61 / f64x8::splat(8.0);
            let t123 = t5 * t25;
            let t124 = param_kappa * param_kappa;
            let t125 = t26 * t124;
            let t126 = t56 * t56;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t37 * v_rho0;
            let t130 = f64x8::splat(1.0) / t39 / t128;
            let t135 = param_alpha * param_alpha;
            let t137 = t30 * t30;
            let t138 = t28 * t135 * t137;
            let t140 = f64x8::splat(1.0) / t33 / t32;
            let t141 = v_sigma0 * v_sigma0;
            let t142 = t140 * t141;
            let t143 = t37 * t37;
            let t144 = t143 * t37;
            let t146 = f64x8::splat(1.0) / t38 / t144;
            let t147 = t46 * t46;
            let t148 = f64x8::splat(1.0) / t147;
            let t154 = (-t31 * t36 * t130 * t47 / f64x8::splat(9.0) + t138 * t142 * t146 * t148 / f64x8::splat(216.0)) * t30;
            let t157 = t36 * t130;
            let t160 = t154 * t43 / f64x8::splat(24.0) - t53 * t157 / f64x8::splat(9.0);
            let t161 = t127 * t160;
            let t162 = t125 * t161;
            let t166 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t113 * t61 - t122 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t162));
            let t167 = t67 * t106;
            let t169 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t167)));
            let t172 = ((t71).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t169));
            let t173 = t172 * t26;
            let t177 = t74 * t118;
            let t180 = t5 * t177 * t100 / f64x8::splat(8.0);
            let t182 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t173 * t100 - t180));
            let tvrho0 = t65 + t104 + t6 * (t166 + t182);
            acc_vrho_0 = tvrho0;
            let t186 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t107)));
            let t189 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t186));
            let t190 = t189 * t26;
            let t195 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t190 * t61 - t122));
            let t197 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t167)));
            let t200 = ((t71).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t197));
            let t201 = t200 * t26;
            let t205 = t5 * t74;
            let t206 = t95 * t95;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t77 * v_rho1;
            let t210 = f64x8::splat(1.0) / t79 / t208;
            let t215 = v_sigma2 * v_sigma2;
            let t216 = t140 * t215;
            let t217 = t77 * t77;
            let t218 = t217 * t77;
            let t220 = f64x8::splat(1.0) / t78 / t218;
            let t221 = t85 * t85;
            let t222 = f64x8::splat(1.0) / t221;
            let t228 = (-t31 * t76 * t210 * t86 / f64x8::splat(9.0) + t138 * t216 * t220 * t222 / f64x8::splat(216.0)) * t30;
            let t231 = t76 * t210;
            let t234 = t228 * t82 / f64x8::splat(24.0) - t92 * t231 / f64x8::splat(9.0);
            let t235 = t207 * t234;
            let t236 = t125 * t235;
            let t240 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t201 * t100 - t180 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t236));
            let tvrho1 = t65 + t104 + t6 * (t195 + t240);
            acc_vrho_1 = tvrho1;
            let t243 = t35 * t41;
            let t248 = t143 * v_rho0;
            let t250 = f64x8::splat(1.0) / t38 / t248;
            let t256 = (t31 * t243 * t47 / f64x8::splat(24.0) - t138 * t140 * v_sigma0 * t250 * t148 / f64x8::splat(576.0)) * t30;
            let t260 = t53 * t243 / f64x8::splat(24.0) + t256 * t43 / f64x8::splat(24.0);
            let t261 = t127 * t260;
            let t262 = t125 * t261;
            let t265 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t262));
            let tvsigma0 = t6 * t265;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t266 = t35 * t81;
            let t271 = t217 * v_rho1;
            let t273 = f64x8::splat(1.0) / t78 / t271;
            let t279 = (t31 * t266 * t86 / f64x8::splat(24.0) - t138 * t140 * v_sigma2 * t273 * t222 / f64x8::splat(576.0)) * t30;
            let t283 = t92 * t266 / f64x8::splat(24.0) + t279 * t82 / f64x8::splat(24.0);
            let t284 = t207 * t283;
            let t285 = t125 * t284;
            let t288 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t285));
            let tvsigma2 = t6 * t288;
            acc_vsigma_2 = tvsigma2;
            let t291 = t23 * t23;
            let t292 = f64x8::splat(1.0) / t291;
            let t293 = t109 * t109;
            let t296 = t105 * t6;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t16 * t297;
            let t301 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t106 + f64x8::splat(2.0) * t298)));
            let t305 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t292 * t293 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t301));
            let t306 = t305 * t26;
            let t310 = t112 * t118;
            let t312 = t5 * t310 * t61;
            let t314 = t5 * t112;
            let t318 = f64x8::splat(1.0) / t117 / t6;
            let t319 = t25 * t318;
            let t322 = t5 * t319 * t61 / f64x8::splat(12.0);
            let t323 = t118 * t124;
            let t324 = t323 * t161;
            let t325 = t123 * t324;
            let t328 = f64x8::splat(1.0) / t126 / t56;
            let t329 = t160 * t160;
            let t330 = t328 * t329;
            let t331 = t125 * t330;
            let t335 = f64x8::splat(1.0) / t39 / t143;
            let t340 = t143 * t128;
            let t342 = f64x8::splat(1.0) / t38 / t340;
            let t348 = t28 * t135 * param_alpha;
            let t349 = t32 * t32;
            let t350 = f64x8::splat(1.0) / t349;
            let t351 = t348 * t350;
            let t352 = t141 * v_sigma0;
            let t353 = t143 * t143;
            let t354 = t353 * t37;
            let t355 = f64x8::splat(1.0) / t354;
            let t358 = f64x8::splat(1.0) / t147 / t46;
            let t363 = (f64x8::splat(11.0) / f64x8::splat(27.0) * t31 * t36 * t335 * t47 - t138 * t142 * t342 * t148 / f64x8::splat(24.0) + t351 * t352 * t355 * t358 / f64x8::splat(162.0)) * t30;
            let t368 = t36 * t335;
            let t371 = t363 * t43 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t154 * t157 + f64x8::splat(11.0) / f64x8::splat(27.0) * t53 * t368;
            let t372 = t127 * t371;
            let t373 = t125 * t372;
            let t377 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t306 * t61 - t312 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t314 * t162 + t322 - t325 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t123 * t331 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t373));
            let t378 = t72 * t72;
            let t379 = f64x8::splat(1.0) / t378;
            let t380 = t169 * t169;
            let t383 = t67 * t297;
            let t386 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t106 + f64x8::splat(2.0) * t383)));
            let t390 = ((t71).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t379 * t380 + f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t386));
            let t391 = t390 * t26;
            let t395 = t172 * t118;
            let t397 = t5 * t395 * t100;
            let t399 = t74 * t318;
            let t402 = t5 * t399 * t100 / f64x8::splat(12.0);
            let t404 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t391 * t100 - t397 / f64x8::splat(4.0) + t402));
            let tv2rho20 = f64x8::splat(2.0) * t166 + f64x8::splat(2.0) * t182 + t6 * (t377 + t404);
            acc_v2rho2_0 = tv2rho20;
            let t407 = t292 * t186;
            let t411 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t298)));
            let t415 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t407 * t109 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t411));
            let t416 = t415 * t26;
            let t420 = t189 * t118;
            let t422 = t5 * t420 * t61;
            let t424 = t5 * t189;
            let t430 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t416 * t61 - t422 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t424 * t162 - t312 / f64x8::splat(8.0) + t322 - t325 / f64x8::splat(8.0)));
            let t431 = t379 * t197;
            let t435 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t383)));
            let t439 = ((t71).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t431 * t169 + f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t435));
            let t440 = t439 * t26;
            let t444 = t200 * t118;
            let t446 = t5 * t444 * t100;
            let t449 = t5 * t172;
            let t452 = t323 * t235;
            let t453 = t205 * t452;
            let t456 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t440 * t100 - t446 / f64x8::splat(8.0) - t397 / f64x8::splat(8.0) + t402 - f64x8::splat(3.0) / f64x8::splat(8.0) * t449 * t236 - t453 / f64x8::splat(8.0)));
            let tv2rho21 = t166 + t182 + t195 + t240 + t6 * (t430 + t456);
            acc_v2rho2_1 = tv2rho21;
            let t461 = t186 * t186;
            let t466 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t106 + f64x8::splat(2.0) * t298)));
            let t470 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t292 * t461 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t466));
            let t471 = t470 * t26;
            let t477 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t471 * t61 - t422 / f64x8::splat(4.0) + t322));
            let t478 = t197 * t197;
            let t483 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t106 + f64x8::splat(2.0) * t383)));
            let t487 = ((t71).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t379 * t478 + f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t483));
            let t488 = t487 * t26;
            let t493 = t5 * t200;
            let t498 = f64x8::splat(1.0) / t206 / t95;
            let t499 = t234 * t234;
            let t500 = t498 * t499;
            let t501 = t125 * t500;
            let t505 = f64x8::splat(1.0) / t79 / t217;
            let t510 = t217 * t208;
            let t512 = f64x8::splat(1.0) / t78 / t510;
            let t517 = t215 * v_sigma2;
            let t518 = t217 * t217;
            let t519 = t518 * t77;
            let t520 = f64x8::splat(1.0) / t519;
            let t523 = f64x8::splat(1.0) / t221 / t85;
            let t528 = (f64x8::splat(11.0) / f64x8::splat(27.0) * t31 * t76 * t505 * t86 - t138 * t216 * t512 * t222 / f64x8::splat(24.0) + t351 * t517 * t520 * t523 / f64x8::splat(162.0)) * t30;
            let t533 = t76 * t505;
            let t536 = t528 * t82 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t228 * t231 + f64x8::splat(11.0) / f64x8::splat(27.0) * t92 * t533;
            let t537 = t207 * t536;
            let t538 = t125 * t537;
            let t542 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t488 * t100 - t446 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t493 * t236 + t402 - t453 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t205 * t501 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t538));
            let tv2rho22 = f64x8::splat(2.0) * t195 + f64x8::splat(2.0) * t240 + t6 * (t477 + t542);
            acc_v2rho2_2 = tv2rho22;
            let t547 = t323 * t261;
            let t549 = t123 * t547 / f64x8::splat(8.0);
            let t550 = t5 * t27;
            let t551 = t124 * t328;
            let t552 = t260 * t160;
            let t553 = t551 * t552;
            let t556 = t35 * t130;
            let t560 = t140 * t146;
            let t561 = t148 * v_sigma0;
            let t565 = t353 * v_rho0;
            let t566 = f64x8::splat(1.0) / t565;
            let t572 = (-t31 * t556 * t47 / f64x8::splat(9.0) + t138 * t560 * t561 / f64x8::splat(72.0) - t351 * t141 * t566 * t358 / f64x8::splat(432.0)) * t30;
            let t581 = t572 * t43 / f64x8::splat(24.0) - t256 * t157 / f64x8::splat(9.0) + t154 * t243 / f64x8::splat(24.0) - t53 * t556 / f64x8::splat(9.0);
            let t582 = t127 * t581;
            let t583 = t125 * t582;
            let t587 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t314 * t262 - t549 + f64x8::splat(3.0) / f64x8::splat(4.0) * t550 * t553 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t583));
            let tv2rhosigma0 = t6 * t587 + t265;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t591 = t323 * t284;
            let t593 = t205 * t591 / f64x8::splat(8.0);
            let t595 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t449 * t285 - t593));
            let tv2rhosigma2 = t6 * t595 + t288;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t600 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t424 * t262 - t549));
            let tv2rhosigma3 = t6 * t600 + t265;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t604 = t5 * t75;
            let t605 = t124 * t498;
            let t606 = t283 * t234;
            let t607 = t605 * t606;
            let t610 = t35 * t210;
            let t614 = t140 * t220;
            let t615 = t222 * v_sigma2;
            let t619 = t518 * v_rho1;
            let t620 = f64x8::splat(1.0) / t619;
            let t626 = (-t31 * t610 * t86 / f64x8::splat(9.0) + t138 * t614 * t615 / f64x8::splat(72.0) - t351 * t215 * t620 * t523 / f64x8::splat(432.0)) * t30;
            let t635 = t626 * t82 / f64x8::splat(24.0) - t279 * t231 / f64x8::splat(9.0) + t228 * t266 / f64x8::splat(24.0) - t92 * t610 / f64x8::splat(9.0);
            let t636 = t207 * t635;
            let t637 = t125 * t636;
            let t641 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t493 * t285 - t593 + f64x8::splat(3.0) / f64x8::splat(4.0) * t604 * t607 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t637));
            let tv2rhosigma5 = t6 * t641 + t288;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t643 = t260 * t260;
            let t644 = t328 * t643;
            let t645 = t125 * t644;
            let t652 = f64x8::splat(1.0) / t353;
            let t658 = (-t138 * t140 * t250 * t148 / f64x8::splat(288.0) + t351 * v_sigma0 * t652 * t358 / f64x8::splat(1152.0)) * t30;
            let t663 = t658 * t43 / f64x8::splat(24.0) + t256 * t243 / f64x8::splat(12.0);
            let t664 = t127 * t663;
            let t665 = t125 * t664;
            let t669 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t123 * t645 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t665));
            let tv2sigma20 = t6 * t669;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t670 = t283 * t283;
            let t671 = t498 * t670;
            let t672 = t125 * t671;
            let t679 = f64x8::splat(1.0) / t518;
            let t685 = (-t138 * t140 * t273 * t222 / f64x8::splat(288.0) + t351 * v_sigma2 * t679 * t523 / f64x8::splat(1152.0)) * t30;
            let t690 = t685 * t82 / f64x8::splat(24.0) + t279 * t266 / f64x8::splat(12.0);
            let t691 = t207 * t690;
            let t692 = t125 * t691;
            let t696 = ((t66).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t205 * t672 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t692));
            let tv2sigma25 = t6 * t696;
            acc_v2sigma2_5 = tv2sigma25;
            let t700 = f64x8::splat(1.0) / t291 / t19;
            let t701 = t293 * t109;
            let t704 = t292 * t109;
            let t707 = t105 * t105;
            let t708 = f64x8::splat(1.0) / t707;
            let t709 = t16 * t708;
            let t712 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t297 - f64x8::splat(6.0) * t709)));
            let t716 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t700 * t701 + f64x8::splat(4.0) / f64x8::splat(3.0) * t704 * t301 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t712));
            let t721 = t323 * t330;
            let t722 = t123 * t721;
            let t724 = t126 * t126;
            let t725 = f64x8::splat(1.0) / t724;
            let t726 = t329 * t160;
            let t727 = t725 * t726;
            let t728 = t125 * t727;
            let t732 = t551 * t160 * t371;
            let t737 = t5 * t305;
            let t740 = t314 * t324;
            let t744 = t318 * t124;
            let t745 = t744 * t161;
            let t746 = t123 * t745;
            let t748 = t323 * t372;
            let t749 = t123 * t748;
            let t752 = f64x8::splat(1.0) / t39 / t248;
            let t758 = f64x8::splat(1.0) / t38 / t353;
            let t763 = t353 * t128;
            let t764 = f64x8::splat(1.0) / t763;
            let t769 = t135 * t135;
            let t770 = t28 * t769;
            let t771 = t141 * t141;
            let t772 = t350 * t771;
            let t773 = t770 * t772;
            let t774 = t353 * t248;
            let t776 = f64x8::splat(1.0) / t39 / t774;
            let t777 = t147 * t147;
            let t778 = f64x8::splat(1.0) / t777;
            let t780 = t30 * t35;
            let t785 = (-f64x8::splat(154.0) / f64x8::splat(81.0) * t31 * t36 * t752 * t47 + f64x8::splat(341.0) / f64x8::splat(972.0) * t138 * t142 * t758 * t148 - f64x8::splat(19.0) / f64x8::splat(162.0) * t351 * t352 * t764 * t358 + t773 * t776 * t778 * t780 / f64x8::splat(486.0)) * t30;
            let t792 = t36 * t752;
            let t795 = t785 * t43 / f64x8::splat(24.0) - t363 * t157 / f64x8::splat(3.0) + f64x8::splat(11.0) / f64x8::splat(9.0) * t154 * t368 - f64x8::splat(154.0) / f64x8::splat(81.0) * t53 * t792;
            let t796 = t127 * t795;
            let t797 = t125 * t796;
            let t802 = t5 * t305 * t118 * t61;
            let t806 = t5 * t112 * t318 * t61;
            let t809 = f64x8::splat(1.0) / t117 / t105;
            let t813 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t25 * t809 * t61;
            let t814 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t716 * t26 * t61 + f64x8::splat(3.0) / f64x8::splat(4.0) * t722 - f64x8::splat(9.0) / f64x8::splat(4.0) * t123 * t728 + f64x8::splat(9.0) / f64x8::splat(4.0) * t550 * t732 + f64x8::splat(9.0) / f64x8::splat(4.0) * t314 * t331 - f64x8::splat(9.0) / f64x8::splat(8.0) * t737 * t162 - f64x8::splat(3.0) / f64x8::splat(4.0) * t740 - f64x8::splat(9.0) / f64x8::splat(8.0) * t314 * t373 + t746 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t749 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t797 - f64x8::splat(3.0) / f64x8::splat(8.0) * t802 + t806 / f64x8::splat(4.0) - t813;
            let t815 = ((t1).select(f64x8::splat(0.0), t814));
            let t817 = f64x8::splat(1.0) / t378 / t70;
            let t818 = t380 * t169;
            let t821 = t379 * t169;
            let t824 = t67 * t708;
            let t827 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t297 - f64x8::splat(6.0) * t824)));
            let t831 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t817 * t818 + f64x8::splat(4.0) / f64x8::splat(3.0) * t821 * t386 + f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t827));
            let t838 = t5 * t390 * t118 * t100;
            let t842 = t5 * t172 * t318 * t100;
            let t847 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t74 * t809 * t100;
            let t849 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t831 * t26 * t100 - f64x8::splat(3.0) / f64x8::splat(8.0) * t838 + t842 / f64x8::splat(4.0) - t847));
            let tv3rho30 = f64x8::splat(3.0) * t377 + f64x8::splat(3.0) * t404 + t6 * (t815 + t849);
            acc_v3rho3_0 = tv3rho30;
            let t852 = f64x8::splat(2.0) * t430;
            let t853 = f64x8::splat(2.0) * t456;
            let t854 = t700 * t186;
            let t857 = t292 * t411;
            let t862 = f64x8::splat(2.0) * t297;
            let t863 = f64x8::splat(6.0) * t709;
            let t865 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t862 - t863)));
            let t869 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t854 * t293 + f64x8::splat(8.0) / f64x8::splat(9.0) * t857 * t109 + f64x8::splat(4.0) / f64x8::splat(9.0) * t407 * t301 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t865));
            let t877 = t5 * t415 * t118 * t61 / f64x8::splat(4.0);
            let t878 = t5 * t415;
            let t883 = t5 * t189 * t318 * t61;
            let t886 = t424 * t324 / f64x8::splat(4.0);
            let t897 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t869 * t26 * t61 - t877 - f64x8::splat(3.0) / f64x8::splat(4.0) * t878 * t162 + t883 / f64x8::splat(12.0) - t886 + f64x8::splat(3.0) / f64x8::splat(4.0) * t424 * t331 - f64x8::splat(3.0) / f64x8::splat(8.0) * t424 * t373 - t802 / f64x8::splat(8.0) + t806 / f64x8::splat(6.0) - t740 / f64x8::splat(4.0) - t813 + t746 / f64x8::splat(6.0) + t722 / f64x8::splat(4.0) - t749 / f64x8::splat(8.0);
            let t898 = ((t1).select(f64x8::splat(0.0), t897));
            let t899 = t817 * t197;
            let t902 = t379 * t435;
            let t907 = f64x8::splat(6.0) * t824;
            let t909 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t862 - t907)));
            let t913 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t899 * t380 + f64x8::splat(8.0) / f64x8::splat(9.0) * t902 * t169 + f64x8::splat(4.0) / f64x8::splat(9.0) * t431 * t386 + f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t909));
            let t921 = t5 * t439 * t118 * t100 / f64x8::splat(4.0);
            let t924 = t5 * t200 * t318 * t100;
            let t928 = t5 * t390;
            let t932 = t449 * t452 / f64x8::splat(4.0);
            let t933 = t744 * t235;
            let t934 = t205 * t933;
            let t937 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t913 * t26 * t100 - t921 + t924 / f64x8::splat(12.0) - t838 / f64x8::splat(8.0) + t842 / f64x8::splat(6.0) - t847 - f64x8::splat(3.0) / f64x8::splat(8.0) * t928 * t236 - t932 + t934 / f64x8::splat(12.0)));
            let tv3rho31 = t377 + t404 + t852 + t853 + t6 * (t898 + t937);
            acc_v3rho3_1 = tv3rho31;
            let t940 = t700 * t461;
            let t945 = t292 * t466;
            let t949 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t862 - t863)));
            let t953 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t940 * t109 + f64x8::splat(8.0) / f64x8::splat(9.0) * t407 * t411 + f64x8::splat(4.0) / f64x8::splat(9.0) * t945 * t109 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t949));
            let t960 = t5 * t470 * t118 * t61;
            let t962 = t5 * t470;
            let t969 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t953 * t26 * t61 - t960 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t962 * t162 - t877 + t883 / f64x8::splat(6.0) - t886 + t806 / f64x8::splat(12.0) - t813 + t746 / f64x8::splat(12.0)));
            let t970 = t817 * t478;
            let t975 = t379 * t483;
            let t979 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t862 - t907)));
            let t983 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t970 * t169 + f64x8::splat(8.0) / f64x8::splat(9.0) * t431 * t435 + f64x8::splat(4.0) / f64x8::splat(9.0) * t975 * t169 + f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t979));
            let t990 = t5 * t487 * t118 * t100;
            let t993 = t5 * t439;
            let t996 = t493 * t452;
            let t1002 = t323 * t500;
            let t1003 = t205 * t1002;
            let t1007 = t323 * t537;
            let t1008 = t205 * t1007;
            let t1010 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t983 * t26 * t100 - t990 / f64x8::splat(8.0) - t921 + t924 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t993 * t236 - t996 / f64x8::splat(4.0) + t842 / f64x8::splat(12.0) - t847 - t932 + t934 / f64x8::splat(6.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t449 * t501 + t1003 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t449 * t538 - t1008 / f64x8::splat(8.0);
            let t1011 = ((t66).select(f64x8::splat(0.0), t1010));
            let tv3rho32 = t852 + t853 + t477 + t542 + t6 * (t969 + t1011);
            acc_v3rho3_2 = tv3rho32;
            let t1016 = t461 * t186;
            let t1023 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t297 - f64x8::splat(6.0) * t709)));
            let t1027 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t700 * t1016 + f64x8::splat(4.0) / f64x8::splat(3.0) * t407 * t466 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1023));
            let t1035 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1027 * t26 * t61 - f64x8::splat(3.0) / f64x8::splat(8.0) * t960 + t883 / f64x8::splat(4.0) - t813));
            let t1036 = t478 * t197;
            let t1043 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t297 - f64x8::splat(6.0) * t824)));
            let t1047 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t817 * t1036 + f64x8::splat(4.0) / f64x8::splat(3.0) * t431 * t483 + f64x8::splat(4.0) / f64x8::splat(3.0) * t72 * t1043));
            let t1052 = t206 * t206;
            let t1053 = f64x8::splat(1.0) / t1052;
            let t1054 = t499 * t234;
            let t1055 = t1053 * t1054;
            let t1056 = t125 * t1055;
            let t1060 = t605 * t234 * t536;
            let t1069 = t5 * t487;
            let t1075 = f64x8::splat(1.0) / t79 / t271;
            let t1081 = f64x8::splat(1.0) / t78 / t518;
            let t1086 = t518 * t208;
            let t1087 = f64x8::splat(1.0) / t1086;
            let t1092 = t215 * t215;
            let t1093 = t350 * t1092;
            let t1094 = t770 * t1093;
            let t1095 = t518 * t271;
            let t1097 = f64x8::splat(1.0) / t79 / t1095;
            let t1098 = t221 * t221;
            let t1099 = f64x8::splat(1.0) / t1098;
            let t1105 = (-f64x8::splat(154.0) / f64x8::splat(81.0) * t31 * t76 * t1075 * t86 + f64x8::splat(341.0) / f64x8::splat(972.0) * t138 * t216 * t1081 * t222 - f64x8::splat(19.0) / f64x8::splat(162.0) * t351 * t517 * t1087 * t523 + t1094 * t1097 * t1099 * t780 / f64x8::splat(486.0)) * t30;
            let t1112 = t76 * t1075;
            let t1115 = t1105 * t82 / f64x8::splat(24.0) - t528 * t231 / f64x8::splat(3.0) + f64x8::splat(11.0) / f64x8::splat(9.0) * t228 * t533 - f64x8::splat(154.0) / f64x8::splat(81.0) * t92 * t1112;
            let t1116 = t207 * t1115;
            let t1117 = t125 * t1116;
            let t1122 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1047 * t26 * t100 - f64x8::splat(9.0) / f64x8::splat(4.0) * t205 * t1056 + f64x8::splat(9.0) / f64x8::splat(4.0) * t604 * t1060 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1008 + f64x8::splat(9.0) / f64x8::splat(4.0) * t493 * t501 + t934 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t996 + f64x8::splat(3.0) / f64x8::splat(4.0) * t1003 - f64x8::splat(9.0) / f64x8::splat(8.0) * t1069 * t236 - f64x8::splat(9.0) / f64x8::splat(8.0) * t493 * t538 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t1117 + t924 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t990 - t847;
            let t1123 = ((t66).select(f64x8::splat(0.0), t1122));
            let tv3rho33 = f64x8::splat(3.0) * t477 + f64x8::splat(3.0) * t542 + t6 * (t1035 + t1123);
            acc_v3rho3_3 = tv3rho33;
            let t1129 = t314 * t547;
            let t1131 = t5 * t113;
            let t1136 = t744 * t261;
            let t1138 = t123 * t1136 / f64x8::splat(12.0);
            let t1139 = t5 * t119;
            let t1140 = t1139 * t553;
            let t1142 = t323 * t582;
            let t1143 = t123 * t1142;
            let t1145 = t124 * t725;
            let t1147 = t1145 * t260 * t329;
            let t1151 = t551 * t581 * t160;
            let t1155 = t551 * t260 * t371;
            let t1158 = t35 * t335;
            let t1162 = t140 * t342;
            let t1166 = t355 * t358;
            let t1170 = t350 * t352;
            let t1172 = t353 * t143;
            let t1174 = f64x8::splat(1.0) / t39 / t1172;
            let t1180 = (f64x8::splat(11.0) / f64x8::splat(27.0) * t31 * t1158 * t47 - f64x8::splat(65.0) / f64x8::splat(648.0) * t138 * t1162 * t561 + f64x8::splat(17.0) / f64x8::splat(432.0) * t351 * t1166 * t141 - t770 * t1170 * t1174 * t778 * t780 / f64x8::splat(1296.0)) * t30;
            let t1193 = t1180 * t43 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t572 * t157 + f64x8::splat(11.0) / f64x8::splat(27.0) * t256 * t368 + t363 * t243 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t154 * t556 + f64x8::splat(11.0) / f64x8::splat(27.0) * t53 * t1158;
            let t1194 = t127 * t1193;
            let t1195 = t125 * t1194;
            let t1198 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t737 * t262 - t1129 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t1131 * t553 - f64x8::splat(3.0) / f64x8::splat(4.0) * t314 * t583 + t1138 + t1140 / f64x8::splat(2.0) - t1143 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(4.0) * t550 * t1147 + f64x8::splat(3.0) / f64x8::splat(2.0) * t550 * t1151 + f64x8::splat(3.0) / f64x8::splat(4.0) * t550 * t1155 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t1195;
            let t1199 = ((t1).select(f64x8::splat(0.0), t1198));
            let tv3rho2sigma0 = t6 * t1199 + f64x8::splat(2.0) * t587;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1204 = t449 * t591;
            let t1206 = t744 * t284;
            let t1208 = t205 * t1206 / f64x8::splat(12.0);
            let t1210 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t928 * t285 - t1204 / f64x8::splat(4.0) + t1208));
            let tv3rho2sigma2 = t6 * t1210 + f64x8::splat(2.0) * t595;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1214 = t424 * t547;
            let t1216 = t5 * t190;
            let t1225 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t878 * t262 - t1214 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t1216 * t553 - f64x8::splat(3.0) / f64x8::splat(8.0) * t424 * t583 - t1129 / f64x8::splat(8.0) + t1138 + t1140 / f64x8::splat(4.0) - t1143 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1225 + t587 + t600;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1229 = t493 * t591;
            let t1232 = t5 * t173;
            let t1235 = t5 * t177;
            let t1236 = t1235 * t607;
            let t1240 = t323 * t636;
            let t1241 = t205 * t1240;
            let t1244 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t993 * t285 - t1229 / f64x8::splat(8.0) - t1204 / f64x8::splat(8.0) + t1208 + f64x8::splat(3.0) / f64x8::splat(4.0) * t1232 * t607 + t1236 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t449 * t637 - t1241 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1244 + t595 + t641;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1251 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t962 * t262 - t1214 / f64x8::splat(4.0) + t1138));
            let tv3rho2sigma6 = t6 * t1251 + f64x8::splat(2.0) * t600;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1257 = t5 * t201;
            let t1264 = t124 * t1053;
            let t1266 = t1264 * t283 * t499;
            let t1270 = t605 * t635 * t234;
            let t1274 = t605 * t283 * t536;
            let t1277 = t35 * t505;
            let t1281 = t140 * t512;
            let t1285 = t520 * t523;
            let t1289 = t350 * t517;
            let t1291 = t518 * t217;
            let t1293 = f64x8::splat(1.0) / t79 / t1291;
            let t1299 = (f64x8::splat(11.0) / f64x8::splat(27.0) * t31 * t1277 * t86 - f64x8::splat(65.0) / f64x8::splat(648.0) * t138 * t1281 * t615 + f64x8::splat(17.0) / f64x8::splat(432.0) * t351 * t1285 * t215 - t770 * t1289 * t1293 * t1099 * t780 / f64x8::splat(1296.0)) * t30;
            let t1312 = t1299 * t82 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t626 * t231 + f64x8::splat(11.0) / f64x8::splat(27.0) * t279 * t533 + t528 * t266 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t228 * t610 + f64x8::splat(11.0) / f64x8::splat(27.0) * t92 * t1277;
            let t1313 = t207 * t1312;
            let t1314 = t125 * t1313;
            let t1317 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t1069 * t285 - t1229 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t1257 * t607 - f64x8::splat(3.0) / f64x8::splat(4.0) * t493 * t637 + t1208 + t1236 / f64x8::splat(2.0) - t1241 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(4.0) * t604 * t1266 + f64x8::splat(3.0) / f64x8::splat(2.0) * t604 * t1270 + f64x8::splat(3.0) / f64x8::splat(4.0) * t604 * t1274 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t1314;
            let t1318 = ((t66).select(f64x8::splat(0.0), t1317));
            let tv3rho2sigma8 = t6 * t1318 + f64x8::splat(2.0) * t641;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1322 = t323 * t644;
            let t1324 = t123 * t1322 / f64x8::splat(4.0);
            let t1326 = t1145 * t643 * t160;
            let t1330 = t551 * t260 * t581;
            let t1335 = t323 * t664;
            let t1337 = t123 * t1335 / f64x8::splat(8.0);
            let t1339 = t551 * t663 * t160;
            let t1349 = t350 * t141;
            let t1352 = f64x8::splat(1.0) / t39 / t763;
            let t1358 = (t138 * t560 * t148 / f64x8::splat(54.0) - f64x8::splat(5.0) / f64x8::splat(432.0) * t351 * t566 * t358 * v_sigma0 + t770 * t1349 * t1352 * t778 * t780 / f64x8::splat(3456.0)) * t30;
            let t1367 = t1358 * t43 / f64x8::splat(24.0) - t658 * t157 / f64x8::splat(9.0) + t572 * t243 / f64x8::splat(12.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t256 * t556;
            let t1368 = t127 * t1367;
            let t1369 = t125 * t1368;
            let t1373 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t314 * t645 + t1324 - f64x8::splat(9.0) / f64x8::splat(4.0) * t550 * t1326 + f64x8::splat(3.0) / f64x8::splat(2.0) * t550 * t1330 - f64x8::splat(3.0) / f64x8::splat(8.0) * t314 * t665 - t1337 + f64x8::splat(3.0) / f64x8::splat(4.0) * t550 * t1339 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t1369));
            let tv3rhosigma20 = t6 * t1373 + t669;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1377 = t323 * t671;
            let t1379 = t205 * t1377 / f64x8::splat(4.0);
            let t1382 = t323 * t691;
            let t1384 = t205 * t1382 / f64x8::splat(8.0);
            let t1386 = ((t66).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t449 * t672 + t1379 - f64x8::splat(3.0) / f64x8::splat(8.0) * t449 * t692 - t1384));
            let tv3rhosigma25 = t6 * t1386 + t696;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1393 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t424 * t645 + t1324 - f64x8::splat(3.0) / f64x8::splat(8.0) * t424 * t665 - t1337));
            let tv3rhosigma26 = t6 * t1393 + t669;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1398 = t1264 * t670 * t234;
            let t1402 = t605 * t283 * t635;
            let t1408 = t605 * t690 * t234;
            let t1418 = t350 * t215;
            let t1421 = f64x8::splat(1.0) / t79 / t1086;
            let t1427 = (t138 * t614 * t222 / f64x8::splat(54.0) - f64x8::splat(5.0) / f64x8::splat(432.0) * t351 * t620 * t523 * v_sigma2 + t770 * t1418 * t1421 * t1099 * t780 / f64x8::splat(3456.0)) * t30;
            let t1436 = t1427 * t82 / f64x8::splat(24.0) - t685 * t231 / f64x8::splat(9.0) + t626 * t266 / f64x8::splat(12.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t279 * t610;
            let t1437 = t207 * t1436;
            let t1438 = t125 * t1437;
            let t1442 = ((t66).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t493 * t672 + t1379 - f64x8::splat(9.0) / f64x8::splat(4.0) * t604 * t1398 + f64x8::splat(3.0) / f64x8::splat(2.0) * t604 * t1402 - f64x8::splat(3.0) / f64x8::splat(8.0) * t493 * t692 - t1384 + f64x8::splat(3.0) / f64x8::splat(4.0) * t604 * t1408 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t1438));
            let tv3rhosigma211 = t6 * t1442 + t696;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1444 = t643 * t260;
            let t1445 = t725 * t1444;
            let t1446 = t125 * t1445;
            let t1449 = t260 * t663;
            let t1450 = t551 * t1449;
            let t1457 = t350 * v_sigma0;
            let t1462 = f64x8::splat(1.0) / t39 / t354 * t778 * t780;
            let t1466 = (t348 * t350 * t652 * t358 / f64x8::splat(384.0) - t770 * t1457 * t1462 / f64x8::splat(9216.0)) * t30;
            let t1471 = t1466 * t43 / f64x8::splat(24.0) + t658 * t243 / f64x8::splat(8.0);
            let t1472 = t127 * t1471;
            let t1473 = t125 * t1472;
            let t1477 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(9.0) / f64x8::splat(4.0) * t123 * t1446 + f64x8::splat(9.0) / f64x8::splat(4.0) * t550 * t1450 - f64x8::splat(3.0) / f64x8::splat(8.0) * t123 * t1473));
            let tv3sigma30 = t6 * t1477;
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
            let t1478 = t670 * t283;
            let t1479 = t1053 * t1478;
            let t1480 = t125 * t1479;
            let t1483 = t283 * t690;
            let t1484 = t605 * t1483;
            let t1491 = t350 * v_sigma2;
            let t1496 = f64x8::splat(1.0) / t79 / t519 * t1099 * t780;
            let t1500 = (t348 * t350 * t679 * t523 / f64x8::splat(384.0) - t770 * t1491 * t1496 / f64x8::splat(9216.0)) * t30;
            let t1505 = t1500 * t82 / f64x8::splat(24.0) + t685 * t266 / f64x8::splat(8.0);
            let t1506 = t207 * t1505;
            let t1507 = t125 * t1506;
            let t1511 = ((t66).select(f64x8::splat(0.0), -f64x8::splat(9.0) / f64x8::splat(4.0) * t205 * t1480 + f64x8::splat(9.0) / f64x8::splat(4.0) * t604 * t1484 - f64x8::splat(3.0) / f64x8::splat(8.0) * t205 * t1507));
            let tv3sigma39 = t6 * t1511;
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
