//! GGA_K_LGAP lxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lgap_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = t25 * t25;
            let t28 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t31 = param_mu_0 * t26 / t29;
            let t32 = ((v_sigma).sqrt());
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = t32 * t33;
            let t36 = f64x8::splat(1.0) / t21 / v_rho;
            let t41 = param_mu_1 * t25;
            let t42 = t29 * t29;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t41 * t43;
            let t45 = t33 * t33;
            let t46 = v_sigma * t45;
            let t47 = v_rho * v_rho;
            let t49 = f64x8::splat(1.0) / t22 / t47;
            let t55 = param_mu_2 / t28;
            let t56 = t32 * v_sigma;
            let t57 = t47 * t47;
            let t58 = f64x8::splat(1.0) / t57;
            let t63 = (simd::exp(-t31 * t34 * t36 / f64x8::splat(12.0) - t44 * t46 * t49 / f64x8::splat(24.0) - t55 * t56 * t58 / f64x8::splat(24.0)));
            let t66 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t63);
            let t70 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t66));
            let tzk0 = f64x8::splat(2.0) * t70;
            acc_zk = tzk0;
            let t71 = f64x8::splat(1.0) / t21;
            let t72 = t20 * t71;
            let t76 = t7 * t20;
            let t77 = t22 * param_kappa;
            let t79 = f64x8::splat(1.0) / t21 / t47;
            let t83 = t47 * v_rho;
            let t85 = f64x8::splat(1.0) / t22 / t83;
            let t89 = t57 * v_rho;
            let t90 = f64x8::splat(1.0) / t89;
            let t94 = t31 * t34 * t79 / f64x8::splat(9.0) + t44 * t46 * t85 / f64x8::splat(9.0) + t55 * t56 * t90 / f64x8::splat(6.0);
            let t95 = t94 * t63;
            let t100 = ((t2).select(f64x8::splat(0.0), t7 * t72 * t66 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t95));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t100 + f64x8::splat(2.0) * t70;
            acc_vrho = tvrho0;
            let t103 = f64x8::splat(1.0) / t32;
            let t104 = t103 * t33;
            let t108 = t43 * t45;
            let t115 = -t31 * t104 * t36 / f64x8::splat(24.0) - t41 * t108 * t49 / f64x8::splat(24.0) - t55 * t32 * t58 / f64x8::splat(16.0);
            let t116 = t115 * t63;
            let t120 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t116));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t120;
            acc_vsigma = tvsigma0;
            let t123 = t20 * t36;
            let t127 = t71 * param_kappa;
            let t132 = f64x8::splat(1.0) / t21 / t83;
            let t137 = f64x8::splat(1.0) / t22 / t57;
            let t141 = t57 * t47;
            let t142 = f64x8::splat(1.0) / t141;
            let t146 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t31 * t34 * t132 - f64x8::splat(11.0) / f64x8::splat(27.0) * t44 * t46 * t137 - f64x8::splat(5.0) / f64x8::splat(6.0) * t55 * t56 * t142;
            let t147 = t146 * t63;
            let t151 = t94 * t94;
            let t152 = t151 * t63;
            let t157 = ((t2).select(f64x8::splat(0.0), -t7 * t123 * t66 / f64x8::splat(30.0) - t76 * t127 * t95 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t147 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t152));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t157 + f64x8::splat(4.0) * t100;
            acc_v2rho2 = tv2rho20;
            let t172 = t31 * t104 * t79 / f64x8::splat(18.0) + t41 * t108 * t85 / f64x8::splat(9.0) + t55 * t32 * t90 / f64x8::splat(4.0);
            let t173 = t172 * t63;
            let t177 = t7 * t23;
            let t178 = param_kappa * t115;
            let t179 = t178 * t95;
            let t183 = ((t2).select(f64x8::splat(0.0), -t76 * t127 * t116 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t173 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t179));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t183 + f64x8::splat(2.0) * t120;
            acc_v2rhosigma = tv2rhosigma0;
            let t186 = f64x8::splat(1.0) / t56;
            let t187 = t186 * t33;
            let t194 = t31 * t187 * t36 / f64x8::splat(48.0) - t55 * t103 * t58 / f64x8::splat(32.0);
            let t195 = t194 * t63;
            let t198 = t115 * t115;
            let t199 = t198 * t63;
            let t204 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t195 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t199));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t204;
            acc_v2sigma2 = tv2sigma20;
            let t211 = t36 * param_kappa;
            let t222 = f64x8::splat(1.0) / t21 / t57;
            let t227 = f64x8::splat(1.0) / t22 / t89;
            let t232 = f64x8::splat(1.0) / t57 / t83;
            let t236 = f64x8::splat(70.0) / f64x8::splat(81.0) * t31 * t34 * t222 + f64x8::splat(154.0) / f64x8::splat(81.0) * t44 * t46 * t227 + f64x8::splat(5.0) * t55 * t56 * t232;
            let t237 = t236 * t63;
            let t241 = param_kappa * t146;
            let t242 = t241 * t95;
            let t246 = t151 * t94 * t63;
            let t251 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(45.0) * t7 * t20 * t79 * t66 + t76 * t211 * t95 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t76 * t127 * t147 - f64x8::splat(3.0) / f64x8::splat(10.0) * t76 * t127 * t152 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t237 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t242 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t246));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t251 + f64x8::splat(6.0) * t157;
            acc_v3rho3 = tv3rho30;
            let t261 = t7 * t72;
            let t273 = -f64x8::splat(7.0) / f64x8::splat(54.0) * t31 * t104 * t132 - f64x8::splat(11.0) / f64x8::splat(27.0) * t41 * t108 * t137 - f64x8::splat(5.0) / f64x8::splat(4.0) * t55 * t32 * t142;
            let t274 = t273 * t63;
            let t278 = param_kappa * t172;
            let t279 = t278 * t95;
            let t282 = t178 * t147;
            let t285 = t178 * t152;
            let t289 = ((t2).select(f64x8::splat(0.0), t76 * t211 * t116 / f64x8::splat(30.0) - t76 * t127 * t173 / f64x8::splat(5.0) - t261 * t179 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t274 - f64x8::splat(3.0) / f64x8::splat(10.0) * t177 * t279 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t282 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t285));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t289 + f64x8::splat(4.0) * t183;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t301 = -t31 * t187 * t79 / f64x8::splat(36.0) + t55 * t103 * t90 / f64x8::splat(8.0);
            let t302 = t301 * t63;
            let t306 = param_kappa * t194;
            let t307 = t306 * t95;
            let t313 = t178 * t173;
            let t316 = param_kappa * t198;
            let t317 = t316 * t95;
            let t321 = ((t2).select(f64x8::splat(0.0), -t76 * t127 * t195 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t302 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t307 - t76 * t127 * t199 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t177 * t313 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t317));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t321 + f64x8::splat(2.0) * t204;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t324 = v_sigma * v_sigma;
            let t326 = f64x8::splat(1.0) / t32 / t324;
            let t327 = t326 * t33;
            let t334 = -t31 * t327 * t36 / f64x8::splat(32.0) + t55 * t186 * t58 / f64x8::splat(64.0);
            let t335 = t334 * t63;
            let t339 = t306 * t116;
            let t342 = t198 * t115;
            let t343 = t342 * t63;
            let t348 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t335 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t339 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t343));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t348;
            acc_v3sigma3 = tv3sigma30;
            let t355 = t79 * param_kappa;
            let t383 = t57 * t57;
            let t397 = t146 * t146;
            let t405 = t151 * t151;
            let t410 = -f64x8::splat(14.0) / f64x8::splat(135.0) * t7 * t20 * t132 * t66 - f64x8::splat(8.0) / f64x8::splat(45.0) * t76 * t355 * t95 + t76 * t211 * t147 / f64x8::splat(5.0) + t76 * t211 * t152 / f64x8::splat(5.0) - f64x8::splat(2.0) / f64x8::splat(5.0) * t76 * t127 * t237 - f64x8::splat(6.0) / f64x8::splat(5.0) * t261 * t242 - f64x8::splat(2.0) / f64x8::splat(5.0) * t76 * t127 * t246 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * (-f64x8::splat(910.0) / f64x8::splat(243.0) * t31 * t34 / t21 / t89 - f64x8::splat(2618.0) / f64x8::splat(243.0) * t44 * t46 / t22 / t141 - f64x8::splat(35.0) * t55 * t56 / t383) * t63 - f64x8::splat(3.0) / f64x8::splat(5.0) * t177 * param_kappa * t236 * t95 - f64x8::splat(9.0) / f64x8::splat(20.0) * t76 * t77 * t397 * t63 - f64x8::splat(9.0) / f64x8::splat(10.0) * t177 * t241 * t152 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t405 * t63;
            let t411 = ((t2).select(f64x8::splat(0.0), t410));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t411 + f64x8::splat(8.0) * t251;
            acc_v4rho4 = tv4rho40;
            let t468 = -f64x8::splat(3.0) / f64x8::splat(10.0) * t261 * t282 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * param_kappa * t273 * t95 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t278 * t147 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t178 * t237 - f64x8::splat(2.0) / f64x8::splat(45.0) * t76 * t355 * t116 + t7 * t123 * t179 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(5.0) * t261 * t279 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * (f64x8::splat(35.0) / f64x8::splat(81.0) * t31 * t104 * t222 + f64x8::splat(154.0) / f64x8::splat(81.0) * t41 * t108 * t227 + f64x8::splat(15.0) / f64x8::splat(2.0) * t55 * t32 * t232) * t63 + t76 * t211 * t173 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t76 * t127 * t274 - f64x8::splat(3.0) / f64x8::splat(10.0) * t261 * t285 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t278 * t152 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t178 * t146 * t94 * t63 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t178 * t246;
            let t469 = ((t2).select(f64x8::splat(0.0), t468));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t469 + f64x8::splat(6.0) * t289;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t492 = param_kappa * t301;
            let t509 = t172 * t172;
            let t527 = t76 * t211 * t195 / f64x8::splat(30.0) - t76 * t127 * t302 / f64x8::splat(5.0) - t261 * t307 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * (f64x8::splat(7.0) / f64x8::splat(108.0) * t31 * t187 * t132 - f64x8::splat(5.0) / f64x8::splat(8.0) * t55 * t103 * t142) * t63 - f64x8::splat(3.0) / f64x8::splat(10.0) * t177 * t492 * t95 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t306 * t147 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t306 * t152 + t76 * t211 * t199 / f64x8::splat(30.0) - f64x8::splat(2.0) / f64x8::splat(5.0) * t261 * t313 - t261 * t317 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(10.0) * t76 * t77 * t509 * t63 - f64x8::splat(3.0) / f64x8::splat(5.0) * t177 * t178 * t95 * t172 - f64x8::splat(3.0) / f64x8::splat(10.0) * t177 * t178 * t274 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t316 * t147 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t316 * t152;
            let t528 = ((t2).select(f64x8::splat(0.0), t527));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t528 + f64x8::splat(4.0) * t321;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t545 = param_kappa * t334;
            let t573 = ((t2).select(f64x8::splat(0.0), -t76 * t127 * t335 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * (t31 * t327 * t79 / f64x8::splat(24.0) - t55 * t186 * t90 / f64x8::splat(16.0)) * t63 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t545 * t95 - f64x8::splat(3.0) / f64x8::splat(10.0) * t261 * t339 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t492 * t116 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t306 * t173 - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t306 * t115 * t94 * t63 - t76 * t127 * t343 / f64x8::splat(10.0) - f64x8::splat(9.0) / f64x8::splat(20.0) * t177 * t316 * t173 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * param_kappa * t342 * t95));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t573 + f64x8::splat(2.0) * t348;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t594 = t194 * t194;
            let t602 = t198 * t198;
            let t608 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * (f64x8::splat(5.0) / f64x8::splat(64.0) * t31 / t32 / t324 / v_sigma * t33 * t36 - f64x8::splat(3.0) / f64x8::splat(128.0) * t55 * t326 * t58) * t63 - f64x8::splat(3.0) / f64x8::splat(5.0) * t177 * t545 * t116 - f64x8::splat(9.0) / f64x8::splat(20.0) * t76 * t77 * t594 * t63 - f64x8::splat(9.0) / f64x8::splat(10.0) * t177 * t306 * t199 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t602 * t63));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t608;
            acc_v4sigma4 = tv4sigma40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
