//! GGA_K_LGAP kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap.c`
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
pub fn gga_k_lgap_kxc_pol(
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
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu_0 = f64x8::splat(param_mu_0);
    let param_mu_1 = f64x8::splat(param_mu_1);
    let param_mu_2 = f64x8::splat(param_mu_2);
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
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = t33 * t33;
            let t35 = param_mu_0 * t34;
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = ((v_sigma0).sqrt());
            let t40 = t38 * t39;
            let t41 = (simd::cbrt(v_rho0));
            let t43 = f64x8::splat(1.0) / t41 / v_rho0;
            let t48 = param_mu_1 * t33;
            let t49 = t37 * t37;
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t50 * v_sigma0;
            let t52 = v_rho0 * v_rho0;
            let t53 = t41 * t41;
            let t55 = f64x8::splat(1.0) / t53 / t52;
            let t61 = param_mu_2 / t36;
            let t62 = t39 * v_sigma0;
            let t63 = t52 * t52;
            let t64 = f64x8::splat(1.0) / t63;
            let t69 = (simd::exp(-t35 * t40 * t43 / f64x8::splat(12.0) - t48 * t51 * t55 / f64x8::splat(24.0) - t61 * t62 * t64 / f64x8::splat(48.0)));
            let t72 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t69);
            let t76 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t72));
            let t77 = (v_rho1).simd_le(dens_threshold);
            let t78 = -t17;
            let t80 = ((t15).select(t12, (t11).select(t16, t78 * t8)));
            let t81 = f64x8::splat(1.0) + t80;
            let t82 = (t81).simd_le(zeta_threshold);
            let t83 = (simd::cbrt(t81));
            let t84 = t83 * t83;
            let t86 = ((t82).select(t24, t84 * t81));
            let t87 = t86 * t30;
            let t88 = ((v_sigma2).sqrt());
            let t89 = t38 * t88;
            let t90 = (simd::cbrt(v_rho1));
            let t92 = f64x8::splat(1.0) / t90 / v_rho1;
            let t96 = t50 * v_sigma2;
            let t97 = v_rho1 * v_rho1;
            let t98 = t90 * t90;
            let t100 = f64x8::splat(1.0) / t98 / t97;
            let t104 = t88 * v_sigma2;
            let t105 = t97 * t97;
            let t106 = f64x8::splat(1.0) / t105;
            let t111 = (simd::exp(-t35 * t89 * t92 / f64x8::splat(12.0) - t48 * t96 * t100 / f64x8::splat(24.0) - t61 * t104 * t106 / f64x8::splat(48.0)));
            let t114 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t111);
            let t118 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t87 * t114));
            let tzk0 = t76 + t118;
            acc_zk = tzk0;
            let t119 = t7 * t7;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t17 * t120;
            let t123 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t121)));
            let t126 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t123));
            let t127 = t126 * t30;
            let t131 = f64x8::splat(1.0) / t29;
            let t132 = t28 * t131;
            let t135 = t6 * t132 * t72 / f64x8::splat(10.0);
            let t136 = t6 * t28;
            let t137 = t30 * param_kappa;
            let t139 = f64x8::splat(1.0) / t41 / t52;
            let t143 = t52 * v_rho0;
            let t145 = f64x8::splat(1.0) / t53 / t143;
            let t149 = t63 * v_rho0;
            let t150 = f64x8::splat(1.0) / t149;
            let t154 = t35 * t40 * t139 / f64x8::splat(9.0) + t48 * t51 * t145 / f64x8::splat(9.0) + t61 * t62 * t150 / f64x8::splat(12.0);
            let t155 = t154 * t69;
            let t156 = t137 * t155;
            let t160 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t127 * t72 + t135 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t156));
            let t161 = t78 * t120;
            let t163 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t161)));
            let t166 = ((t82).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t163));
            let t167 = t166 * t30;
            let t171 = t86 * t131;
            let t174 = t6 * t171 * t114 / f64x8::splat(10.0);
            let t176 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t167 * t114 + t174));
            let tvrho0 = t76 + t118 + t7 * (t160 + t176);
            acc_vrho_0 = tvrho0;
            let t180 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t121)));
            let t183 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t180));
            let t184 = t183 * t30;
            let t189 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t184 * t72 + t135));
            let t191 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t161)));
            let t194 = ((t82).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t191));
            let t195 = t194 * t30;
            let t199 = t6 * t86;
            let t201 = f64x8::splat(1.0) / t90 / t97;
            let t205 = t97 * v_rho1;
            let t207 = f64x8::splat(1.0) / t98 / t205;
            let t211 = t105 * v_rho1;
            let t212 = f64x8::splat(1.0) / t211;
            let t216 = t35 * t89 * t201 / f64x8::splat(9.0) + t48 * t96 * t207 / f64x8::splat(9.0) + t61 * t104 * t212 / f64x8::splat(12.0);
            let t217 = t216 * t111;
            let t218 = t137 * t217;
            let t222 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t195 * t114 + t174 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t218));
            let tvrho1 = t76 + t118 + t7 * (t189 + t222);
            acc_vrho_1 = tvrho1;
            let t225 = f64x8::splat(1.0) / t39;
            let t226 = t38 * t225;
            let t236 = -t35 * t226 * t43 / f64x8::splat(24.0) - t48 * t50 * t55 / f64x8::splat(24.0) - t61 * t39 * t64 / f64x8::splat(32.0);
            let t237 = t236 * t69;
            let t238 = t137 * t237;
            let t241 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t238));
            let tvsigma0 = t7 * t241;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t242 = f64x8::splat(1.0) / t88;
            let t243 = t38 * t242;
            let t253 = -t35 * t243 * t92 / f64x8::splat(24.0) - t48 * t50 * t100 / f64x8::splat(24.0) - t61 * t88 * t106 / f64x8::splat(32.0);
            let t254 = t253 * t111;
            let t255 = t137 * t254;
            let t258 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t255));
            let tvsigma2 = t7 * t258;
            acc_vsigma_2 = tvsigma2;
            let t261 = f64x8::splat(1.0) / t25;
            let t262 = t123 * t123;
            let t265 = t119 * t7;
            let t266 = f64x8::splat(1.0) / t265;
            let t267 = t17 * t266;
            let t270 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t120 + f64x8::splat(2.0) * t267)));
            let t274 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t261 * t262 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t270));
            let t275 = t274 * t30;
            let t279 = t126 * t131;
            let t281 = t6 * t279 * t72;
            let t283 = t6 * t126;
            let t287 = f64x8::splat(1.0) / t29 / t7;
            let t288 = t28 * t287;
            let t291 = t6 * t288 * t72 / f64x8::splat(30.0);
            let t292 = t131 * param_kappa;
            let t293 = t292 * t155;
            let t294 = t136 * t293;
            let t297 = f64x8::splat(1.0) / t41 / t143;
            let t302 = f64x8::splat(1.0) / t53 / t63;
            let t306 = t63 * t52;
            let t307 = f64x8::splat(1.0) / t306;
            let t311 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t35 * t40 * t297 - f64x8::splat(11.0) / f64x8::splat(27.0) * t48 * t51 * t302 - f64x8::splat(5.0) / f64x8::splat(12.0) * t61 * t62 * t307;
            let t312 = t311 * t69;
            let t313 = t137 * t312;
            let t316 = t154 * t154;
            let t317 = t316 * t69;
            let t318 = t137 * t317;
            let t322 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t275 * t72 + t281 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t283 * t156 - t291 - t294 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t313 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t318));
            let t323 = f64x8::splat(1.0) / t83;
            let t324 = t163 * t163;
            let t327 = t78 * t266;
            let t330 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t120 + f64x8::splat(2.0) * t327)));
            let t334 = ((t82).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t323 * t324 + f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t330));
            let t335 = t334 * t30;
            let t339 = t166 * t131;
            let t341 = t6 * t339 * t114;
            let t343 = t86 * t287;
            let t346 = t6 * t343 * t114 / f64x8::splat(30.0);
            let t348 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t335 * t114 + t341 / f64x8::splat(5.0) - t346));
            let tv2rho20 = f64x8::splat(2.0) * t160 + f64x8::splat(2.0) * t176 + t7 * (t322 + t348);
            acc_v2rho2_0 = tv2rho20;
            let t351 = t261 * t180;
            let t355 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t267)));
            let t359 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t351 * t123 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t355));
            let t360 = t359 * t30;
            let t364 = t183 * t131;
            let t366 = t6 * t364 * t72;
            let t368 = t6 * t183;
            let t374 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t360 * t72 + t366 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t368 * t156 + t281 / f64x8::splat(10.0) - t291 - t294 / f64x8::splat(10.0)));
            let t375 = t323 * t191;
            let t379 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t327)));
            let t383 = ((t82).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t375 * t163 + f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t379));
            let t384 = t383 * t30;
            let t388 = t194 * t131;
            let t390 = t6 * t388 * t114;
            let t393 = t6 * t166;
            let t396 = t292 * t217;
            let t397 = t199 * t396;
            let t400 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t384 * t114 + t390 / f64x8::splat(10.0) + t341 / f64x8::splat(10.0) - t346 - f64x8::splat(3.0) / f64x8::splat(20.0) * t393 * t218 - t397 / f64x8::splat(10.0)));
            let tv2rho21 = t160 + t176 + t189 + t222 + t7 * (t374 + t400);
            acc_v2rho2_1 = tv2rho21;
            let t405 = t180 * t180;
            let t410 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t120 + f64x8::splat(2.0) * t267)));
            let t414 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t261 * t405 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t410));
            let t415 = t414 * t30;
            let t421 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t415 * t72 + t366 / f64x8::splat(5.0) - t291));
            let t422 = t191 * t191;
            let t427 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t120 + f64x8::splat(2.0) * t327)));
            let t431 = ((t82).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t323 * t422 + f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t427));
            let t432 = t431 * t30;
            let t437 = t6 * t194;
            let t442 = f64x8::splat(1.0) / t90 / t205;
            let t447 = f64x8::splat(1.0) / t98 / t105;
            let t451 = t105 * t97;
            let t452 = f64x8::splat(1.0) / t451;
            let t456 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t35 * t89 * t442 - f64x8::splat(11.0) / f64x8::splat(27.0) * t48 * t96 * t447 - f64x8::splat(5.0) / f64x8::splat(12.0) * t61 * t104 * t452;
            let t457 = t456 * t111;
            let t458 = t137 * t457;
            let t461 = t216 * t216;
            let t462 = t461 * t111;
            let t463 = t137 * t462;
            let t467 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t432 * t114 + t390 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t437 * t218 - t346 - t397 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t458 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t463));
            let tv2rho22 = f64x8::splat(2.0) * t189 + f64x8::splat(2.0) * t222 + t7 * (t421 + t467);
            acc_v2rho2_2 = tv2rho22;
            let t472 = t292 * t237;
            let t474 = t136 * t472 / f64x8::splat(10.0);
            let t484 = t35 * t226 * t139 / f64x8::splat(18.0) + t48 * t50 * t145 / f64x8::splat(9.0) + t61 * t39 * t150 / f64x8::splat(8.0);
            let t485 = t484 * t69;
            let t486 = t137 * t485;
            let t489 = t6 * t31;
            let t490 = param_kappa * t236;
            let t491 = t490 * t155;
            let t495 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t283 * t238 - t474 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t486 - f64x8::splat(3.0) / f64x8::splat(20.0) * t489 * t491));
            let tv2rhosigma0 = t7 * t495 + t241;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t499 = t292 * t254;
            let t501 = t199 * t499 / f64x8::splat(10.0);
            let t503 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t393 * t255 - t501));
            let tv2rhosigma2 = t7 * t503 + t258;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t508 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t368 * t238 - t474));
            let tv2rhosigma3 = t7 * t508 + t241;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t521 = t35 * t243 * t201 / f64x8::splat(18.0) + t48 * t50 * t207 / f64x8::splat(9.0) + t61 * t88 * t212 / f64x8::splat(8.0);
            let t522 = t521 * t111;
            let t523 = t137 * t522;
            let t526 = t6 * t87;
            let t527 = param_kappa * t253;
            let t528 = t527 * t217;
            let t532 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t437 * t255 - t501 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t523 - f64x8::splat(3.0) / f64x8::splat(20.0) * t526 * t528));
            let tv2rhosigma5 = t7 * t532 + t258;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t534 = f64x8::splat(1.0) / t62;
            let t535 = t38 * t534;
            let t542 = t35 * t535 * t43 / f64x8::splat(48.0) - t61 * t225 * t64 / f64x8::splat(64.0);
            let t543 = t542 * t69;
            let t544 = t137 * t543;
            let t546 = t236 * t236;
            let t547 = t546 * t69;
            let t548 = t137 * t547;
            let t552 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t544 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t548));
            let tv2sigma20 = t7 * t552;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t553 = f64x8::splat(1.0) / t104;
            let t554 = t38 * t553;
            let t561 = t35 * t554 * t92 / f64x8::splat(48.0) - t61 * t242 * t106 / f64x8::splat(64.0);
            let t562 = t561 * t111;
            let t563 = t137 * t562;
            let t565 = t253 * t253;
            let t566 = t565 * t111;
            let t567 = t137 * t566;
            let t571 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t563 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t567));
            let tv2sigma25 = t7 * t571;
            acc_v2sigma2_5 = tv2sigma25;
            let t576 = t292 * t317;
            let t577 = t136 * t576;
            let t579 = param_kappa * t311;
            let t580 = t579 * t155;
            let t584 = t316 * t154 * t69;
            let t585 = t137 * t584;
            let t589 = f64x8::splat(1.0) / t25 / t20;
            let t590 = t262 * t123;
            let t593 = t261 * t123;
            let t596 = t119 * t119;
            let t597 = f64x8::splat(1.0) / t596;
            let t598 = t17 * t597;
            let t601 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(6.0) * t266 - f64x8::splat(6.0) * t598)));
            let t605 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t589 * t590 + f64x8::splat(10.0) / f64x8::splat(3.0) * t593 * t270 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t601));
            let t610 = t6 * t274;
            let t613 = t283 * t293;
            let t617 = t287 * param_kappa;
            let t618 = t617 * t155;
            let t619 = t136 * t618;
            let t621 = t292 * t312;
            let t622 = t136 * t621;
            let t625 = f64x8::splat(1.0) / t41 / t63;
            let t630 = f64x8::splat(1.0) / t53 / t149;
            let t635 = f64x8::splat(1.0) / t63 / t143;
            let t639 = f64x8::splat(70.0) / f64x8::splat(81.0) * t35 * t40 * t625 + f64x8::splat(154.0) / f64x8::splat(81.0) * t48 * t51 * t630 + f64x8::splat(5.0) / f64x8::splat(2.0) * t61 * t62 * t635;
            let t640 = t639 * t69;
            let t641 = t137 * t640;
            let t646 = t6 * t274 * t131 * t72;
            let t650 = t6 * t126 * t287 * t72;
            let t653 = f64x8::splat(1.0) / t29 / t119;
            let t657 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t28 * t653 * t72;
            let t658 = -f64x8::splat(9.0) / f64x8::splat(20.0) * t283 * t318 - f64x8::splat(3.0) / f64x8::splat(10.0) * t577 - f64x8::splat(9.0) / f64x8::splat(20.0) * t489 * t580 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t585 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t605 * t30 * t72 - f64x8::splat(9.0) / f64x8::splat(20.0) * t610 * t156 - f64x8::splat(3.0) / f64x8::splat(5.0) * t613 - f64x8::splat(9.0) / f64x8::splat(20.0) * t283 * t313 + t619 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t622 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t641 + f64x8::splat(3.0) / f64x8::splat(10.0) * t646 - t650 / f64x8::splat(10.0) + t657;
            let t659 = ((t1).select(f64x8::splat(0.0), t658));
            let t661 = f64x8::splat(1.0) / t83 / t81;
            let t662 = t324 * t163;
            let t665 = t323 * t163;
            let t668 = t78 * t597;
            let t671 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t266 - f64x8::splat(6.0) * t668)));
            let t675 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t661 * t662 + f64x8::splat(10.0) / f64x8::splat(3.0) * t665 * t330 + f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t671));
            let t682 = t6 * t334 * t131 * t114;
            let t686 = t6 * t166 * t287 * t114;
            let t691 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t86 * t653 * t114;
            let t693 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t675 * t30 * t114 + f64x8::splat(3.0) / f64x8::splat(10.0) * t682 - t686 / f64x8::splat(10.0) + t691));
            let tv3rho30 = f64x8::splat(3.0) * t322 + f64x8::splat(3.0) * t348 + t7 * (t659 + t693);
            acc_v3rho3_0 = tv3rho30;
            let t696 = f64x8::splat(2.0) * t374;
            let t697 = f64x8::splat(2.0) * t400;
            let t698 = t589 * t180;
            let t701 = t261 * t355;
            let t706 = f64x8::splat(2.0) * t266;
            let t707 = f64x8::splat(6.0) * t598;
            let t709 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t706 - t707)));
            let t713 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t698 * t262 + f64x8::splat(20.0) / f64x8::splat(9.0) * t701 * t123 + f64x8::splat(10.0) / f64x8::splat(9.0) * t351 * t270 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t709));
            let t721 = t6 * t359 * t131 * t72 / f64x8::splat(5.0);
            let t722 = t6 * t359;
            let t727 = t6 * t183 * t287 * t72;
            let t730 = t368 * t293 / f64x8::splat(5.0);
            let t741 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t713 * t30 * t72 + t721 - f64x8::splat(3.0) / f64x8::splat(10.0) * t722 * t156 - t727 / f64x8::splat(30.0) - t730 - f64x8::splat(3.0) / f64x8::splat(20.0) * t368 * t313 - f64x8::splat(3.0) / f64x8::splat(20.0) * t368 * t318 + t646 / f64x8::splat(10.0) - t650 / f64x8::splat(15.0) - t613 / f64x8::splat(5.0) + t657 + t619 / f64x8::splat(15.0) - t622 / f64x8::splat(10.0) - t577 / f64x8::splat(10.0);
            let t742 = ((t1).select(f64x8::splat(0.0), t741));
            let t743 = t661 * t191;
            let t746 = t323 * t379;
            let t751 = f64x8::splat(6.0) * t668;
            let t753 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t706 - t751)));
            let t757 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t743 * t324 + f64x8::splat(20.0) / f64x8::splat(9.0) * t746 * t163 + f64x8::splat(10.0) / f64x8::splat(9.0) * t375 * t330 + f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t753));
            let t765 = t6 * t383 * t131 * t114 / f64x8::splat(5.0);
            let t768 = t6 * t194 * t287 * t114;
            let t772 = t6 * t334;
            let t776 = t393 * t396 / f64x8::splat(5.0);
            let t777 = t617 * t217;
            let t778 = t199 * t777;
            let t781 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t757 * t30 * t114 + t765 - t768 / f64x8::splat(30.0) + t682 / f64x8::splat(10.0) - t686 / f64x8::splat(15.0) + t691 - f64x8::splat(3.0) / f64x8::splat(20.0) * t772 * t218 - t776 + t778 / f64x8::splat(30.0)));
            let tv3rho31 = t322 + t348 + t696 + t697 + t7 * (t742 + t781);
            acc_v3rho3_1 = tv3rho31;
            let t784 = t589 * t405;
            let t789 = t261 * t410;
            let t793 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t706 - t707)));
            let t797 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t784 * t123 + f64x8::splat(20.0) / f64x8::splat(9.0) * t351 * t355 + f64x8::splat(10.0) / f64x8::splat(9.0) * t789 * t123 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t793));
            let t804 = t6 * t414 * t131 * t72;
            let t806 = t6 * t414;
            let t813 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t797 * t30 * t72 + t804 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t806 * t156 + t721 - t727 / f64x8::splat(15.0) - t730 - t650 / f64x8::splat(30.0) + t657 + t619 / f64x8::splat(30.0)));
            let t814 = t661 * t422;
            let t819 = t323 * t427;
            let t823 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t706 - t751)));
            let t827 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t814 * t163 + f64x8::splat(20.0) / f64x8::splat(9.0) * t375 * t379 + f64x8::splat(10.0) / f64x8::splat(9.0) * t819 * t163 + f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t823));
            let t834 = t6 * t431 * t131 * t114;
            let t837 = t6 * t383;
            let t840 = t437 * t396;
            let t846 = t292 * t457;
            let t847 = t199 * t846;
            let t851 = t292 * t462;
            let t852 = t199 * t851;
            let t854 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t827 * t30 * t114 + t834 / f64x8::splat(10.0) + t765 - t768 / f64x8::splat(15.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t837 * t218 - t840 / f64x8::splat(5.0) - t686 / f64x8::splat(30.0) + t691 - t776 + t778 / f64x8::splat(15.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t393 * t458 - t847 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t393 * t463 - t852 / f64x8::splat(10.0);
            let t855 = ((t77).select(f64x8::splat(0.0), t854));
            let tv3rho32 = t696 + t697 + t421 + t467 + t7 * (t813 + t855);
            acc_v3rho3_2 = tv3rho32;
            let t860 = t405 * t180;
            let t867 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t266 - f64x8::splat(6.0) * t598)));
            let t871 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t589 * t860 + f64x8::splat(10.0) / f64x8::splat(3.0) * t351 * t410 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t867));
            let t879 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t871 * t30 * t72 + f64x8::splat(3.0) / f64x8::splat(10.0) * t804 - t727 / f64x8::splat(10.0) + t657));
            let t880 = param_kappa * t456;
            let t881 = t880 * t217;
            let t885 = t461 * t216 * t111;
            let t886 = t137 * t885;
            let t895 = t422 * t191;
            let t902 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(6.0) * t266 - f64x8::splat(6.0) * t668)));
            let t906 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t661 * t895 + f64x8::splat(10.0) / f64x8::splat(3.0) * t375 * t427 + f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t902));
            let t912 = f64x8::splat(1.0) / t90 / t105;
            let t917 = f64x8::splat(1.0) / t98 / t211;
            let t922 = f64x8::splat(1.0) / t105 / t205;
            let t926 = f64x8::splat(70.0) / f64x8::splat(81.0) * t35 * t89 * t912 + f64x8::splat(154.0) / f64x8::splat(81.0) * t48 * t96 * t917 + f64x8::splat(5.0) / f64x8::splat(2.0) * t61 * t104 * t922;
            let t927 = t926 * t111;
            let t928 = t137 * t927;
            let t931 = t6 * t431;
            let t938 = -f64x8::splat(9.0) / f64x8::splat(20.0) * t526 * t881 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t886 - f64x8::splat(3.0) / f64x8::splat(10.0) * t852 - f64x8::splat(9.0) / f64x8::splat(20.0) * t437 * t463 + t778 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(5.0) * t840 - f64x8::splat(3.0) / f64x8::splat(10.0) * t847 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t906 * t30 * t114 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t928 - f64x8::splat(9.0) / f64x8::splat(20.0) * t931 * t218 - f64x8::splat(9.0) / f64x8::splat(20.0) * t437 * t458 + f64x8::splat(3.0) / f64x8::splat(10.0) * t834 + t691 - t768 / f64x8::splat(10.0);
            let t939 = ((t77).select(f64x8::splat(0.0), t938));
            let tv3rho33 = f64x8::splat(3.0) * t421 + f64x8::splat(3.0) * t467 + t7 * (t879 + t939);
            acc_v3rho3_3 = tv3rho33;
            let t945 = t283 * t472;
            let t949 = t6 * t127;
            let t952 = t617 * t237;
            let t954 = t136 * t952 / f64x8::splat(30.0);
            let t955 = t292 * t485;
            let t956 = t136 * t955;
            let t958 = t6 * t132;
            let t959 = t958 * t491;
            let t970 = -f64x8::splat(7.0) / f64x8::splat(54.0) * t35 * t226 * t297 - f64x8::splat(11.0) / f64x8::splat(27.0) * t48 * t50 * t302 - f64x8::splat(5.0) / f64x8::splat(8.0) * t61 * t39 * t307;
            let t971 = t970 * t69;
            let t972 = t137 * t971;
            let t975 = param_kappa * t484;
            let t976 = t975 * t155;
            let t979 = t490 * t312;
            let t982 = t490 * t317;
            let t985 = -f64x8::splat(3.0) / f64x8::splat(20.0) * t610 * t238 - t945 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t283 * t486 - f64x8::splat(3.0) / f64x8::splat(10.0) * t949 * t491 + t954 - t956 / f64x8::splat(5.0) - t959 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t972 - f64x8::splat(3.0) / f64x8::splat(10.0) * t489 * t976 - f64x8::splat(3.0) / f64x8::splat(20.0) * t489 * t979 - f64x8::splat(3.0) / f64x8::splat(20.0) * t489 * t982;
            let t986 = ((t1).select(f64x8::splat(0.0), t985));
            let tv3rho2sigma0 = t7 * t986 + f64x8::splat(2.0) * t495;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t991 = t393 * t499;
            let t993 = t617 * t254;
            let t995 = t199 * t993 / f64x8::splat(30.0);
            let t997 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t772 * t255 - t991 / f64x8::splat(5.0) + t995));
            let tv3rho2sigma2 = t7 * t997 + f64x8::splat(2.0) * t503;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1001 = t368 * t472;
            let t1005 = t6 * t184;
            let t1012 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t722 * t238 - t1001 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t368 * t486 - f64x8::splat(3.0) / f64x8::splat(20.0) * t1005 * t491 - t945 / f64x8::splat(10.0) + t954 - t956 / f64x8::splat(10.0) - t959 / f64x8::splat(10.0)));
            let tv3rho2sigma3 = t7 * t1012 + t495 + t508;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1016 = t437 * t499;
            let t1021 = t292 * t522;
            let t1022 = t199 * t1021;
            let t1024 = t6 * t167;
            let t1027 = t6 * t171;
            let t1028 = t1027 * t528;
            let t1031 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t837 * t255 - t1016 / f64x8::splat(10.0) - t991 / f64x8::splat(10.0) + t995 - f64x8::splat(3.0) / f64x8::splat(20.0) * t393 * t523 - t1022 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t1024 * t528 - t1028 / f64x8::splat(10.0)));
            let tv3rho2sigma5 = t7 * t1031 + t503 + t532;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1038 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t806 * t238 - t1001 / f64x8::splat(5.0) + t954));
            let tv3rho2sigma6 = t7 * t1038 + f64x8::splat(2.0) * t508;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1046 = t6 * t195;
            let t1060 = -f64x8::splat(7.0) / f64x8::splat(54.0) * t35 * t243 * t442 - f64x8::splat(11.0) / f64x8::splat(27.0) * t48 * t50 * t447 - f64x8::splat(5.0) / f64x8::splat(8.0) * t61 * t88 * t452;
            let t1061 = t1060 * t111;
            let t1062 = t137 * t1061;
            let t1065 = param_kappa * t521;
            let t1066 = t1065 * t217;
            let t1069 = t527 * t457;
            let t1072 = t527 * t462;
            let t1075 = -f64x8::splat(3.0) / f64x8::splat(20.0) * t931 * t255 - t1016 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t437 * t523 - f64x8::splat(3.0) / f64x8::splat(10.0) * t1046 * t528 + t995 - t1022 / f64x8::splat(5.0) - t1028 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t1062 - f64x8::splat(3.0) / f64x8::splat(10.0) * t526 * t1066 - f64x8::splat(3.0) / f64x8::splat(20.0) * t526 * t1069 - f64x8::splat(3.0) / f64x8::splat(20.0) * t526 * t1072;
            let t1076 = ((t77).select(f64x8::splat(0.0), t1075));
            let tv3rho2sigma8 = t7 * t1076 + f64x8::splat(2.0) * t532;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1080 = t292 * t543;
            let t1082 = t136 * t1080 / f64x8::splat(10.0);
            let t1089 = -t35 * t535 * t139 / f64x8::splat(36.0) + t61 * t225 * t150 / f64x8::splat(16.0);
            let t1090 = t1089 * t69;
            let t1091 = t137 * t1090;
            let t1094 = param_kappa * t542;
            let t1095 = t1094 * t155;
            let t1100 = t292 * t547;
            let t1102 = t136 * t1100 / f64x8::splat(10.0);
            let t1103 = t490 * t485;
            let t1106 = param_kappa * t546;
            let t1107 = t1106 * t155;
            let t1111 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t283 * t544 - t1082 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t1091 - f64x8::splat(3.0) / f64x8::splat(20.0) * t489 * t1095 - f64x8::splat(3.0) / f64x8::splat(20.0) * t283 * t548 - t1102 - f64x8::splat(3.0) / f64x8::splat(10.0) * t489 * t1103 - f64x8::splat(3.0) / f64x8::splat(20.0) * t489 * t1107));
            let tv3rhosigma20 = t7 * t1111 + t552;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1115 = t292 * t562;
            let t1117 = t199 * t1115 / f64x8::splat(10.0);
            let t1120 = t292 * t566;
            let t1122 = t199 * t1120 / f64x8::splat(10.0);
            let t1124 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t393 * t563 - t1117 - f64x8::splat(3.0) / f64x8::splat(20.0) * t393 * t567 - t1122));
            let tv3rhosigma25 = t7 * t1124 + t571;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1131 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t368 * t544 - t1082 - f64x8::splat(3.0) / f64x8::splat(20.0) * t368 * t548 - t1102));
            let tv3rhosigma26 = t7 * t1131 + t552;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1141 = -t35 * t554 * t201 / f64x8::splat(36.0) + t61 * t242 * t212 / f64x8::splat(16.0);
            let t1142 = t1141 * t111;
            let t1143 = t137 * t1142;
            let t1146 = param_kappa * t561;
            let t1147 = t1146 * t217;
            let t1152 = t527 * t522;
            let t1155 = param_kappa * t565;
            let t1156 = t1155 * t217;
            let t1160 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t437 * t563 - t1117 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t1143 - f64x8::splat(3.0) / f64x8::splat(20.0) * t526 * t1147 - f64x8::splat(3.0) / f64x8::splat(20.0) * t437 * t567 - t1122 - f64x8::splat(3.0) / f64x8::splat(10.0) * t526 * t1152 - f64x8::splat(3.0) / f64x8::splat(20.0) * t526 * t1156));
            let tv3rhosigma211 = t7 * t1160 + t571;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1162 = v_sigma0 * v_sigma0;
            let t1164 = f64x8::splat(1.0) / t39 / t1162;
            let t1165 = t38 * t1164;
            let t1172 = -t35 * t1165 * t43 / f64x8::splat(32.0) + t61 * t534 * t64 / f64x8::splat(128.0);
            let t1173 = t1172 * t69;
            let t1174 = t137 * t1173;
            let t1177 = t1094 * t237;
            let t1180 = t546 * t236;
            let t1181 = t1180 * t69;
            let t1182 = t137 * t1181;
            let t1186 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t1174 - f64x8::splat(9.0) / f64x8::splat(20.0) * t489 * t1177 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t1182));
            let tv3sigma30 = t7 * t1186;
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
            let t1187 = v_sigma2 * v_sigma2;
            let t1189 = f64x8::splat(1.0) / t88 / t1187;
            let t1190 = t38 * t1189;
            let t1197 = -t35 * t1190 * t92 / f64x8::splat(32.0) + t61 * t553 * t106 / f64x8::splat(128.0);
            let t1198 = t1197 * t111;
            let t1199 = t137 * t1198;
            let t1202 = t1146 * t254;
            let t1205 = t565 * t253;
            let t1206 = t1205 * t111;
            let t1207 = t137 * t1206;
            let t1211 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t1199 - f64x8::splat(9.0) / f64x8::splat(20.0) * t526 * t1202 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t1207));
            let tv3sigma39 = t7 * t1211;
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
