//! MGGA_X_TPSS fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`
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
pub fn mgga_x_tpss_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_BLOC_a: f64,
    param_BLOC_b: f64,
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_BLOC_a = f64x8::splat(param_BLOC_a);
    let param_BLOC_b = f64x8::splat(param_BLOC_b);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_e = f64x8::splat(param_e);
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(1.0) / v_rho;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = v_sigma * t21 * t23 / f64x8::splat(8.0);
            let t26 = param_BLOC_b * v_sigma;
            let t30 = param_BLOC_a + t26 * t21 * t23 / f64x8::splat(8.0);
            let t31 = (simd::pow(t25, t30));
            let t32 = param_c * t31;
            let t33 = v_sigma * v_sigma;
            let t34 = v_rho * v_rho;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t33 * t35;
            let t37 = v_tau * v_tau;
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = t36 * t38;
            let t41 = f64x8::splat(1.0) + t39 / f64x8::splat(64.0);
            let t42 = t41 * t41;
            let t43 = f64x8::splat(1.0) / t42;
            let t46 = f64x8::splat(M_CBRT6);
            let t47 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t32 * t43) * t46;
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t47 * t51;
            let t53 = f64x8::splat(M_CBRT2);
            let t54 = t53 * t53;
            let t55 = v_sigma * t54;
            let t56 = t19 * t19;
            let t58 = f64x8::splat(1.0) / t56 / t34;
            let t59 = t55 * t58;
            let t62 = v_tau * t54;
            let t64 = f64x8::splat(1.0) / t56 / v_rho;
            let t67 = t62 * t64 - t59 / f64x8::splat(8.0);
            let t71 = f64x8::splat(5.0) / f64x8::splat(9.0) * t67 * t46 * t51 - f64x8::splat(1.0);
            let t72 = param_b * t67;
            let t73 = t46 * t51;
            let t74 = t73 * t71;
            let t77 = f64x8::splat(5.0) * t72 * t74 + f64x8::splat(9.0);
            let t78 = ((t77).sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t84 = f64x8::splat(27.0) / f64x8::splat(20.0) * t71 * t79 + t73 * t59 / f64x8::splat(36.0);
            let t85 = t84 * t84;
            let t88 = t46 * t46;
            let t90 = f64x8::splat(1.0) / t49 / t48;
            let t91 = t88 * t90;
            let t92 = t33 * t53;
            let t93 = t34 * t34;
            let t94 = t93 * v_rho;
            let t96 = f64x8::splat(1.0) / t19 / t94;
            let t97 = t92 * t96;
            let t100 = f64x8::splat(100.0) * t91 * t97 + f64x8::splat(162.0) * t39;
            let t101 = ((t100).sqrt());
            let t105 = f64x8::splat(1.0) / param_kappa * t88;
            let t106 = t105 * t90;
            let t109 = ((param_e).sqrt());
            let t110 = t109 * t33;
            let t111 = t35 * t38;
            let t114 = param_e * param_mu;
            let t115 = t48 * t48;
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t33 * v_sigma;
            let t118 = t116 * t117;
            let t119 = t93 * t93;
            let t120 = f64x8::splat(1.0) / t119;
            let t124 = t52 * t59 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t85 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t84 * t101 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t106 * t97 + t110 * t111 / f64x8::splat(720.0) + t114 * t118 * t120 / f64x8::splat(576.0);
            let t125 = t109 * t46;
            let t129 = f64x8::splat(1.0) + t125 * t51 * t59 / f64x8::splat(24.0);
            let t130 = t129 * t129;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t124 * t131 + param_kappa;
            let t138 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t133);
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t56;
            let t144 = t18 * t143;
            let t148 = t7 * t18;
            let t149 = param_kappa * param_kappa;
            let t150 = t19 * t149;
            let t151 = t133 * t133;
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t35 * t23;
            let t154 = (simd::ln(t25));
            let t159 = -t26 * t153 * t154 / f64x8::splat(8.0) - t30 * t21;
            let t160 = t159 * t43;
            let t163 = f64x8::splat(1.0) / t42 / t41;
            let t164 = t32 * t163;
            let t165 = t34 * v_rho;
            let t166 = f64x8::splat(1.0) / t165;
            let t167 = t33 * t166;
            let t168 = t167 * t38;
            let t172 = (t32 * t160 + t164 * t168 / f64x8::splat(16.0)) * t46;
            let t173 = t172 * t51;
            let t177 = f64x8::splat(1.0) / t56 / t165;
            let t178 = t55 * t177;
            let t184 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t62 * t58 + t178 / f64x8::splat(3.0);
            let t185 = t184 * t46;
            let t186 = t51 * t79;
            let t190 = f64x8::splat(1.0) / t78 / t77;
            let t191 = t71 * t190;
            let t195 = t91 * t184;
            let t198 = f64x8::splat(5.0) * param_b * t184 * t74 + f64x8::splat(25.0) / f64x8::splat(9.0) * t72 * t195;
            let t203 = f64x8::splat(3.0) / f64x8::splat(4.0) * t185 * t186 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t198 - f64x8::splat(2.0) / f64x8::splat(27.0) * t73 * t178;
            let t208 = f64x8::splat(1.0) / t101;
            let t209 = t84 * t208;
            let t211 = t93 * t34;
            let t213 = f64x8::splat(1.0) / t19 / t211;
            let t214 = t92 * t213;
            let t217 = -f64x8::splat(324.0) * t168 - f64x8::splat(1600.0) / f64x8::splat(3.0) * t91 * t214;
            let t222 = t166 * t38;
            let t225 = t119 * v_rho;
            let t226 = f64x8::splat(1.0) / t225;
            let t230 = t173 * t59 / f64x8::splat(24.0) - t52 * t178 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t203 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t203 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t217 - f64x8::splat(50.0) / f64x8::splat(177147.0) * t106 * t214 - t110 * t222 / f64x8::splat(360.0) - t114 * t118 * t226 / f64x8::splat(72.0);
            let t232 = t130 * t129;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t124 * t233;
            let t235 = t234 * t125;
            let t236 = t51 * v_sigma;
            let t237 = t54 * t177;
            let t238 = t236 * t237;
            let t241 = t230 * t131 + f64x8::splat(2.0) / f64x8::splat(9.0) * t235 * t238;
            let t242 = t152 * t241;
            let t247 = ((t3).select(f64x8::splat(0.0), -t7 * t144 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t150 * t242));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t247 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t250 = param_BLOC_b * t21;
            let t251 = t23 * t154;
            let t254 = f64x8::splat(1.0) / v_sigma;
            let t256 = t250 * t251 / f64x8::splat(8.0) + t30 * t254;
            let t257 = t256 * t43;
            let t258 = t32 * t257;
            let t259 = v_sigma * t35;
            let t260 = t259 * t38;
            let t264 = (t258 - t164 * t260 / f64x8::splat(16.0)) * t46;
            let t265 = t264 * t51;
            let t268 = t51 * t54;
            let t269 = t268 * t58;
            let t272 = t54 * t58;
            let t273 = t73 * t79;
            let t274 = t272 * t273;
            let t276 = param_b * t54;
            let t277 = t276 * t58;
            let t278 = t277 * t74;
            let t280 = t72 * t88;
            let t281 = t90 * t54;
            let t283 = t280 * t281 * t58;
            let t285 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t278 - f64x8::splat(25.0) / f64x8::splat(72.0) * t283;
            let t288 = t272 * t73;
            let t290 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t274 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t285 + t288 / f64x8::splat(36.0);
            let t296 = v_sigma * t53;
            let t297 = t296 * t96;
            let t300 = f64x8::splat(200.0) * t91 * t297 + f64x8::splat(324.0) * t260;
            let t305 = t109 * v_sigma;
            let t308 = t116 * t33;
            let t312 = t265 * t59 / f64x8::splat(24.0) + t47 * t269 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t290 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t290 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t300 + f64x8::splat(25.0) / f64x8::splat(236196.0) * t106 * t297 + t305 * t111 / f64x8::splat(360.0) + t114 * t308 * t120 / f64x8::splat(192.0);
            let t314 = t234 * t109;
            let t317 = t312 * t131 - t314 * t288 / f64x8::splat(12.0);
            let t318 = t152 * t317;
            let t322 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t150 * t318));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t322;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t324 = t7 * t20;
            let t325 = t149 * t152;
            let t326 = t21 * t38;
            let t331 = -t26 * t326 * t154 / f64x8::splat(8.0) - t30 * t23;
            let t332 = t331 * t43;
            let t333 = t32 * t332;
            let t334 = t37 * v_tau;
            let t335 = f64x8::splat(1.0) / t334;
            let t336 = t36 * t335;
            let t340 = (t333 + t164 * t336 / f64x8::splat(16.0)) * t46;
            let t341 = t340 * t51;
            let t344 = t54 * t64;
            let t347 = t276 * t64;
            let t353 = f64x8::splat(5.0) * t347 * t74 + f64x8::splat(25.0) / f64x8::splat(9.0) * t280 * t281 * t64;
            let t356 = f64x8::splat(3.0) / f64x8::splat(4.0) * t344 * t273 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t353;
            let t363 = t35 * t335;
            let t366 = t341 * t59 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t356 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t356 * t101 + f64x8::splat(73.0) / f64x8::splat(600.0) * t209 * t336 - t110 * t363 / f64x8::splat(360.0);
            let t367 = t366 * t131;
            let t368 = t325 * t367;
            let t371 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t324 * t368));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t371;
            acc_vtau = tvtau0;
            let t374 = t18 * t64;
            let t378 = t143 * t149;
            let t383 = f64x8::splat(1.0) / t151 / t133;
            let t384 = t241 * t241;
            let t385 = t383 * t384;
            let t389 = t159 * t159;
            let t390 = t389 * t43;
            let t392 = t166 * t23;
            let t399 = t26 * t392 * t154 / f64x8::splat(4.0) + t26 * t392 / f64x8::splat(4.0) + t30 * t35;
            let t402 = t32 * t159;
            let t403 = t163 * t33;
            let t404 = t403 * t222;
            let t407 = t42 * t42;
            let t408 = f64x8::splat(1.0) / t407;
            let t409 = t32 * t408;
            let t410 = t33 * t33;
            let t411 = f64x8::splat(1.0) / t211;
            let t412 = t410 * t411;
            let t413 = t37 * t37;
            let t414 = f64x8::splat(1.0) / t413;
            let t418 = f64x8::splat(1.0) / t93;
            let t419 = t33 * t418;
            let t420 = t419 * t38;
            let t424 = (t32 * t390 + t32 * t399 * t43 + t402 * t404 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(512.0) * t409 * t412 * t414 - f64x8::splat(3.0) / f64x8::splat(16.0) * t164 * t420) * t46;
            let t425 = t424 * t51;
            let t431 = f64x8::splat(1.0) / t56 / t93;
            let t432 = t55 * t431;
            let t435 = t203 * t203;
            let t440 = f64x8::splat(40.0) / f64x8::splat(9.0) * t62 * t177 - f64x8::splat(11.0) / f64x8::splat(9.0) * t432;
            let t441 = t440 * t46;
            let t444 = t51 * t190;
            let t445 = t444 * t198;
            let t448 = t77 * t77;
            let t450 = f64x8::splat(1.0) / t78 / t448;
            let t451 = t71 * t450;
            let t452 = t198 * t198;
            let t455 = param_b * t440;
            let t458 = t184 * t184;
            let t462 = t91 * t440;
            let t465 = f64x8::splat(5.0) * t455 * t74 + f64x8::splat(50.0) / f64x8::splat(9.0) * param_b * t458 * t91 + f64x8::splat(25.0) / f64x8::splat(9.0) * t72 * t462;
            let t470 = f64x8::splat(3.0) / f64x8::splat(4.0) * t441 * t186 - f64x8::splat(3.0) / f64x8::splat(4.0) * t185 * t445 + f64x8::splat(81.0) / f64x8::splat(80.0) * t451 * t452 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t465 + f64x8::splat(22.0) / f64x8::splat(81.0) * t73 * t432;
            let t475 = t203 * t208;
            let t479 = f64x8::splat(1.0) / t101 / t100;
            let t480 = t84 * t479;
            let t481 = t217 * t217;
            let t485 = t93 * t165;
            let t487 = f64x8::splat(1.0) / t19 / t485;
            let t488 = t92 * t487;
            let t491 = f64x8::splat(972.0) * t420 + f64x8::splat(30400.0) / f64x8::splat(9.0) * t91 * t488;
            let t496 = t418 * t38;
            let t499 = t119 * t34;
            let t500 = f64x8::splat(1.0) / t499;
            let t504 = t425 * t59 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t173 * t178 + f64x8::splat(11.0) / f64x8::splat(27.0) * t52 * t432 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t435 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t470 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t470 * t101 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t475 * t217 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t480 * t481 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t491 + f64x8::splat(950.0) / f64x8::splat(531441.0) * t106 * t488 + t110 * t496 / f64x8::splat(120.0) + t114 * t118 * t500 / f64x8::splat(8.0);
            let t506 = t230 * t233;
            let t507 = t506 * t125;
            let t510 = t130 * t130;
            let t511 = f64x8::splat(1.0) / t510;
            let t512 = t124 * t511;
            let t513 = param_e * t88;
            let t514 = t512 * t513;
            let t515 = t90 * t33;
            let t516 = t53 * t487;
            let t517 = t515 * t516;
            let t520 = t54 * t431;
            let t521 = t236 * t520;
            let t524 = t504 * t131 + f64x8::splat(4.0) / f64x8::splat(9.0) * t507 * t238 + f64x8::splat(4.0) / f64x8::splat(27.0) * t514 * t517 - f64x8::splat(22.0) / f64x8::splat(27.0) * t235 * t521;
            let t525 = t152 * t524;
            let t530 = ((t3).select(f64x8::splat(0.0), t7 * t374 * t138 / f64x8::splat(12.0) - t148 * t378 * t242 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t148 * t150 * t385 - f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t150 * t525));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t530 + f64x8::splat(4.0) * t247;
            acc_v2rho2 = tv2rho20;
            let t536 = t149 * t383;
            let t537 = t317 * t241;
            let t538 = t536 * t537;
            let t541 = t159 * t256;
            let t544 = param_BLOC_b * t35;
            let t549 = -t544 * t251 / f64x8::splat(8.0) - t544 * t23 / f64x8::splat(4.0);
            let t550 = t549 * t43;
            let t552 = t32 * t256;
            let t555 = t163 * v_sigma;
            let t556 = t555 * t111;
            let t559 = f64x8::splat(1.0) / t94;
            let t560 = t117 * t559;
            let t564 = v_sigma * t166;
            let t565 = t564 * t38;
            let t569 = (t32 * t541 * t43 + t32 * t550 + t552 * t404 / f64x8::splat(16.0) - t402 * t556 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(512.0) * t409 * t560 * t414 + t164 * t565 / f64x8::splat(8.0)) * t46;
            let t570 = t569 * t51;
            let t577 = t268 * t177;
            let t582 = t237 * t273;
            let t584 = t272 * t46;
            let t585 = t584 * t445;
            let t587 = t444 * t285;
            let t590 = t285 * t198;
            let t593 = t276 * t177;
            let t594 = t593 * t74;
            let t596 = t277 * t195;
            let t599 = t280 * t281 * t177;
            let t601 = f64x8::splat(5.0) / f64x8::splat(3.0) * t594 - f64x8::splat(25.0) / f64x8::splat(36.0) * t596 + f64x8::splat(25.0) / f64x8::splat(27.0) * t599;
            let t604 = t237 * t73;
            let t606 = t582 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(64.0) * t585 - f64x8::splat(3.0) / f64x8::splat(8.0) * t185 * t587 + f64x8::splat(81.0) / f64x8::splat(80.0) * t451 * t590 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t601 - f64x8::splat(2.0) / f64x8::splat(27.0) * t604;
            let t611 = t290 * t208;
            let t616 = t300 * t217;
            let t620 = t296 * t213;
            let t623 = -f64x8::splat(648.0) * t565 - f64x8::splat(3200.0) / f64x8::splat(3.0) * t91 * t620;
            let t633 = t570 * t59 / f64x8::splat(24.0) - t265 * t178 / f64x8::splat(9.0) + t172 * t269 / f64x8::splat(24.0) - t47 * t577 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t203 * t290 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t606 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t606 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t611 * t217 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t475 * t300 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t480 * t616 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t623 - f64x8::splat(100.0) / f64x8::splat(177147.0) * t106 * t620 - t305 * t222 / f64x8::splat(180.0) - t114 * t308 * t226 / f64x8::splat(24.0);
            let t635 = t312 * t233;
            let t636 = t635 * t125;
            let t639 = t506 * t109;
            let t642 = t90 * t53;
            let t644 = t642 * t213 * v_sigma;
            let t649 = t633 * t131 + f64x8::splat(2.0) / f64x8::splat(9.0) * t636 * t238 - t639 * t288 / f64x8::splat(12.0) - t514 * t644 / f64x8::splat(18.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t314 * t604;
            let t650 = t152 * t649;
            let t655 = ((t3).select(f64x8::splat(0.0), -t148 * t378 * t318 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t324 * t538 - f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t150 * t650));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t655 + f64x8::splat(2.0) * t322;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t658 = t7 * t144;
            let t662 = t536 * t367 * t241;
            let t665 = t159 * t331;
            let t673 = t26 * t111 * t154 / f64x8::splat(8.0) + t26 * t111 / f64x8::splat(4.0);
            let t674 = t673 * t43;
            let t676 = t32 * t331;
            let t679 = t403 * t363;
            let t682 = t410 * t559;
            let t684 = f64x8::splat(1.0) / t413 / v_tau;
            let t688 = t167 * t335;
            let t692 = (t32 * t665 * t43 + t32 * t674 + t676 * t404 / f64x8::splat(16.0) + t402 * t679 / f64x8::splat(16.0) + f64x8::splat(3.0) / f64x8::splat(512.0) * t409 * t682 * t684 - t164 * t688 / f64x8::splat(8.0)) * t46;
            let t693 = t692 * t51;
            let t701 = t344 * t46;
            let t704 = t444 * t353;
            let t707 = t353 * t198;
            let t714 = -f64x8::splat(25.0) / f64x8::splat(3.0) * t278 + f64x8::splat(50.0) / f64x8::splat(9.0) * t347 * t195 - f64x8::splat(125.0) / f64x8::splat(27.0) * t283;
            let t717 = -f64x8::splat(5.0) / f64x8::splat(4.0) * t274 - f64x8::splat(3.0) / f64x8::splat(8.0) * t701 * t445 - f64x8::splat(3.0) / f64x8::splat(8.0) * t185 * t704 + f64x8::splat(81.0) / f64x8::splat(80.0) * t451 * t707 - f64x8::splat(27.0) / f64x8::splat(40.0) * t191 * t714;
            let t722 = t356 * t208;
            let t727 = t480 * t33;
            let t728 = t363 * t217;
            let t733 = t166 * t335;
            let t736 = t693 * t59 / f64x8::splat(24.0) - t341 * t178 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t203 * t356 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t717 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t717 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t722 * t217 + f64x8::splat(73.0) / f64x8::splat(600.0) * t475 * t336 - f64x8::splat(73.0) / f64x8::splat(1200.0) * t727 * t728 - f64x8::splat(73.0) / f64x8::splat(300.0) * t209 * t688 + t110 * t733 / f64x8::splat(180.0);
            let t737 = t736 * t131;
            let t738 = t325 * t737;
            let t742 = f64x8::splat(1.0) / t19 / t165;
            let t743 = t742 * t149;
            let t745 = t148 * t743 * t152;
            let t746 = t366 * t233;
            let t747 = t746 * t109;
            let t748 = t73 * t55;
            let t749 = t747 * t748;
            let t753 = ((t3).select(f64x8::splat(0.0), -t658 * t368 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t324 * t662 - f64x8::splat(3.0) / f64x8::splat(8.0) * t324 * t738 - t745 * t749 / f64x8::splat(12.0)));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t753 + f64x8::splat(2.0) * t371;
            acc_v2rhotau = tv2rhotau0;
            let t756 = t317 * t317;
            let t757 = t383 * t756;
            let t761 = t256 * t256;
            let t762 = t761 * t43;
            let t767 = f64x8::splat(1.0) / t33;
            let t769 = t250 * t23 * t254 / f64x8::splat(4.0) - t30 * t767;
            let t774 = t419 * t414;
            let t777 = t163 * t35;
            let t778 = t777 * t38;
            let t782 = (t32 * t762 + t32 * t769 * t43 - t552 * t556 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(512.0) * t409 * t774 - t32 * t778 / f64x8::splat(16.0)) * t46;
            let t783 = t782 * t51;
            let t788 = t290 * t290;
            let t790 = t584 * t587;
            let t792 = t285 * t285;
            let t795 = t191 * param_b;
            let t796 = t53 * t96;
            let t797 = t91 * t796;
            let t798 = t795 * t797;
            let t800 = f64x8::splat(3.0) / f64x8::splat(32.0) * t790 + f64x8::splat(81.0) / f64x8::splat(80.0) * t451 * t792 - f64x8::splat(15.0) / f64x8::splat(128.0) * t798;
            let t807 = t300 * t300;
            let t812 = f64x8::splat(324.0) * t111 + f64x8::splat(200.0) * t797;
            let t818 = t109 * t35;
            let t821 = t116 * v_sigma;
            let t825 = t783 * t59 / f64x8::splat(24.0) + t264 * t269 / f64x8::splat(12.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t788 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t800 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t800 * t101 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t611 * t300 + f64x8::splat(73.0) / f64x8::splat(388800.0) * t480 * t807 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t209 * t812 + f64x8::splat(25.0) / f64x8::splat(236196.0) * t105 * t642 * t96 + t818 * t38 / f64x8::splat(360.0) + t114 * t821 * t120 / f64x8::splat(96.0);
            let t827 = t635 * t109;
            let t830 = t512 * param_e;
            let t833 = t825 * t131 - t827 * t288 / f64x8::splat(6.0) + t830 * t797 / f64x8::splat(48.0);
            let t834 = t152 * t833;
            let t839 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t148 * t150 * t757 - f64x8::splat(3.0) / f64x8::splat(8.0) * t148 * t150 * t834));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t839;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t842 = t536 * t367 * t317;
            let t845 = t256 * t331;
            let t846 = t845 * t43;
            let t848 = t38 * t154;
            let t853 = -t250 * t848 / f64x8::splat(8.0) - t250 * t38 / f64x8::splat(4.0);
            let t854 = t853 * t43;
            let t860 = t117 * t418;
            let t864 = t259 * t335;
            let t868 = (t32 * t846 + t32 * t854 - t676 * t556 / f64x8::splat(16.0) + t552 * t679 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(512.0) * t409 * t860 * t684 + t164 * t864 / f64x8::splat(8.0)) * t46;
            let t869 = t868 * t51;
            let t878 = t584 * t704;
            let t880 = t353 * t285;
            let t884 = f64x8::splat(1.0) / t19 / t93;
            let t885 = t53 * t884;
            let t886 = t885 * t91;
            let t887 = t795 * t886;
            let t889 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t701 * t587 + f64x8::splat(3.0) / f64x8::splat(64.0) * t878 + f64x8::splat(81.0) / f64x8::splat(80.0) * t451 * t880 + f64x8::splat(15.0) / f64x8::splat(16.0) * t887;
            let t898 = t363 * t300;
            let t905 = t869 * t59 / f64x8::splat(24.0) + t340 * t269 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t290 * t356 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t889 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t889 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t722 * t300 + f64x8::splat(73.0) / f64x8::splat(600.0) * t611 * t336 - f64x8::splat(73.0) / f64x8::splat(1200.0) * t727 * t898 + f64x8::splat(73.0) / f64x8::splat(300.0) * t209 * t864 - t305 * t363 / f64x8::splat(180.0);
            let t906 = t905 * t131;
            let t907 = t325 * t906;
            let t912 = f64x8::splat(1.0) / t19 / t34 * t149;
            let t914 = t148 * t912 * t152;
            let t915 = t73 * t54;
            let t916 = t747 * t915;
            let t920 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t324 * t842 - f64x8::splat(3.0) / f64x8::splat(8.0) * t324 * t907 + t914 * t916 / f64x8::splat(32.0)));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t920;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t922 = t366 * t366;
            let t923 = t922 * t511;
            let t924 = t536 * t923;
            let t927 = t331 * t331;
            let t928 = t927 * t43;
            let t930 = t21 * t335;
            let t937 = t26 * t930 * t154 / f64x8::splat(4.0) + t26 * t930 / f64x8::splat(4.0) + t30 * t38;
            let t942 = t410 * t418;
            let t944 = f64x8::splat(1.0) / t413 / t37;
            let t945 = t942 * t944;
            let t948 = t36 * t414;
            let t952 = (t32 * t928 + t32 * t937 * t43 + t676 * t679 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(512.0) * t409 * t945 - f64x8::splat(3.0) / f64x8::splat(16.0) * t164 * t948) * t46;
            let t953 = t952 * t51;
            let t956 = t356 * t356;
            let t960 = t353 * t353;
            let t963 = t53 * t742;
            let t964 = t963 * t91;
            let t967 = -f64x8::splat(3.0) / f64x8::splat(4.0) * t701 * t704 + f64x8::splat(81.0) / f64x8::splat(80.0) * t451 * t960 - f64x8::splat(15.0) / f64x8::splat(2.0) * t795 * t964;
            let t978 = t35 * t414;
            let t981 = t953 * t59 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t956 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t84 * t967 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t967 * t101 + f64x8::splat(73.0) / f64x8::splat(300.0) * t722 * t336 + f64x8::splat(1971.0) / f64x8::splat(100.0) * t480 * t945 - f64x8::splat(73.0) / f64x8::splat(200.0) * t209 * t948 + t110 * t978 / f64x8::splat(120.0);
            let t982 = t981 * t131;
            let t983 = t325 * t982;
            let t987 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t324 * t924 - f64x8::splat(3.0) / f64x8::splat(8.0) * t324 * t983));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t987;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
