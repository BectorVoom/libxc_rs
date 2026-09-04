//! MGGA_X_TASK fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_task.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_task_fxc_unpol(
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
    param_task_c: f64,
    param_task_bnu_0: f64,
    param_task_bnu_1: f64,
    param_task_bnu_2: f64,
    param_task_bnu_3: f64,
    param_task_bnu_4: f64,
    param_task_anu_0: f64,
    param_task_anu_1: f64,
    param_task_anu_2: f64,
    param_task_h0x: f64,
    param_task_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_task_c = f64x8::splat(param_task_c);
    let param_task_bnu_0 = f64x8::splat(param_task_bnu_0);
    let param_task_bnu_1 = f64x8::splat(param_task_bnu_1);
    let param_task_bnu_2 = f64x8::splat(param_task_bnu_2);
    let param_task_bnu_3 = f64x8::splat(param_task_bnu_3);
    let param_task_bnu_4 = f64x8::splat(param_task_bnu_4);
    let param_task_anu_0 = f64x8::splat(param_task_anu_0);
    let param_task_anu_1 = f64x8::splat(param_task_anu_1);
    let param_task_anu_2 = f64x8::splat(param_task_anu_2);
    let param_task_h0x = f64x8::splat(param_task_h0x);
    let param_task_d = f64x8::splat(param_task_d);
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
            let t12 = t11 + f64x8::splat(1.0);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t26 = t21 / t24;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t32 = t31 * t30;
            let t33 = f64x8::splat(1.0) / t32;
            let t36 = t26 * t29 * t33 / f64x8::splat(24.0);
            let t37 = (f64x8::splat(0.0)).simd_lt(t36);
            let t38 = ((t37).select(t36, f64x8::splat(0.0)));
            let t39 = ((t38).sqrt().sqrt());
            let t42 = (simd::exp(-param_task_c / t39));
            let t44 = ((t37).select(f64x8::splat(1.0) - t42, f64x8::splat(0.0)));
            let t46 = v_tau * v_tau;
            let t47 = t46 * t46;
            let t48 = t47 * t4;
            let t49 = param_task_bnu_0;
            let t50 = param_task_bnu_1;
            let t51 = param_task_bnu_2;
            let t52 = param_task_bnu_3;
            let t53 = param_task_bnu_4;
            let t54 = t49 + t50 + t51 + t52 + t53;
            let t55 = v_rho * v_tau;
            let t59 = f64x8::splat(1.0) / v_rho;
            let t61 = f64x8::splat(1.0) / v_tau;
            let t63 = (f64x8::splat(0.0)).simd_lt((f64x8::splat(0.9999999999) * t55 - f64x8::splat(0.125) * v_sigma) * t59 * t61);
            let t65 = f64x8::splat(8.0) * t55 - v_sigma;
            let t66 = t65 * t59;
            let t69 = ((t63).select(t66 * t61 / f64x8::splat(8.0), f64x8::splat(1e-10)));
            let t70 = t69 * t69;
            let t71 = t70 * t70;
            let t72 = t54 * t71;
            let t75 = t5 * f64x8::splat(M_PI);
            let t76 = t50 / f64x8::splat(2.0);
            let t77 = f64x8::splat(7.0) / f64x8::splat(2.0) * t52;
            let t78 = f64x8::splat(7.0) * t53;
            let t80 = t75 * (t49 + t76 - t51 - t77 - t78);
            let t81 = t31 * v_rho;
            let t82 = t46 * v_tau;
            let t83 = t81 * t82;
            let t84 = t70 * t69;
            let t88 = t30 * v_rho;
            let t89 = t19 * t88;
            let t90 = t5 * t5;
            let t91 = t90 * t22;
            let t92 = t89 * t91;
            let t93 = t4 * t4;
            let t94 = t92 * t93;
            let t97 = t49 - f64x8::splat(5.0) / f64x8::splat(3.0) * t51 + f64x8::splat(35.0) / f64x8::splat(3.0) * t53;
            let t98 = t97 * t46;
            let t99 = t98 * t70;
            let t102 = t30 * t30;
            let t103 = t102 * v_rho;
            let t104 = t22 * t22;
            let t105 = t103 * t104;
            let t106 = t49 - t76 - t51 + t77 - t78;
            let t107 = t105 * t106;
            let t108 = v_tau * t4;
            let t109 = t108 * t69;
            let t113 = t31 * t102 * t30;
            let t115 = t5 * t104 * f64x8::splat(M_PI);
            let t116 = t113 * t115;
            let t117 = t49 - t50 + t51 - t52 + t53;
            let t120 = f64x8::splat(108000.0) * t80 * t83 * t84 + f64x8::splat(29160.0) * t107 * t109 + f64x8::splat(6561.0) * t116 * t117 + f64x8::splat(30000.0) * t48 * t72 + f64x8::splat(48600.0) * t94 * t99;
            let t121 = t81 * t75;
            let t124 = f64x8::splat(9.0) * t121 + f64x8::splat(10.0) * t109;
            let t125 = t124 * t124;
            let t126 = t125 * t125;
            let t127 = f64x8::splat(1.0) / t126;
            let t129 = f64x8::splat(1.0) - t120 * t127;
            let t130 = param_task_anu_0;
            let t131 = param_task_anu_1;
            let t132 = param_task_anu_2;
            let t134 = t91 * (t130 - t131 + t132);
            let t138 = t4 * t75;
            let t140 = t130 - f64x8::splat(3.0) * t132;
            let t143 = f64x8::splat(24.0) * t138 * t140 * t32;
            let t145 = t130 + t131 + t132;
            let t146 = v_sigma * t93 * t145;
            let t149 = f64x8::splat(144.0) * t134 * t19 * t103 + (t143 + t146) * v_sigma;
            let t153 = f64x8::splat(12.0) * t75 * t32 + t4 * v_sigma;
            let t154 = t153 * t153;
            let t155 = f64x8::splat(1.0) / t154;
            let t157 = t149 * t155 - param_task_h0x;
            let t158 = t129 * t157;
            let t159 = (simd::pow(t44, param_task_d));
            let t160 = t158 * t159;
            let t161 = param_task_h0x * t44 + t160;
            let t165 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t161));
            let tzk0 = f64x8::splat(2.0) * t165;
            acc_zk = tzk0;
            let t166 = f64x8::splat(1.0) / t31;
            let t167 = t18 * t166;
            let t173 = param_task_c / t39 / t38;
            let t174 = t31 * t88;
            let t175 = f64x8::splat(1.0) / t174;
            let t179 = ((t37).select(-t26 * t29 * t175 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t180 = t179 * t42;
            let t183 = ((t37).select(-t173 * t180 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t185 = t54 * t84;
            let t186 = f64x8::splat(1.0) / t30;
            let t187 = t65 * t186;
            let t191 = ((t63).select(t59 - t187 * t61 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t192 = t185 * t191;
            let t195 = t31 * t82;
            let t199 = t80 * t81;
            let t200 = t82 * t70;
            let t201 = t200 * t191;
            let t204 = t19 * t30;
            let t205 = t204 * t91;
            let t206 = t205 * t93;
            let t209 = t69 * t191;
            let t210 = t98 * t209;
            let t213 = t102 * t104;
            let t214 = t213 * t106;
            let t217 = t108 * t191;
            let t220 = t31 * t103;
            let t224 = f64x8::splat(43740.0) * t220 * t115 * t117 + f64x8::splat(180000.0) * t80 * t195 * t84 + f64x8::splat(29160.0) * t107 * t217 + f64x8::splat(145800.0) * t214 * t109 + f64x8::splat(120000.0) * t48 * t192 + f64x8::splat(324000.0) * t199 * t201 + f64x8::splat(162000.0) * t206 * t99 + f64x8::splat(97200.0) * t94 * t210;
            let t227 = f64x8::splat(1.0) / t126 / t124;
            let t228 = t120 * t227;
            let t229 = t31 * t75;
            let t232 = f64x8::splat(15.0) * t229 + f64x8::splat(10.0) * t217;
            let t235 = -t224 * t127 + f64x8::splat(4.0) * t228 * t232;
            let t237 = t235 * t157 * t159;
            let t241 = t140 * t81;
            let t245 = f64x8::splat(768.0) * t134 * t19 * t102 + f64x8::splat(64.0) * t138 * t241 * v_sigma;
            let t248 = f64x8::splat(1.0) / t154 / t153;
            let t249 = t149 * t248;
            let t252 = -f64x8::splat(64.0) * t249 * t121 + t245 * t155;
            let t254 = t129 * t252 * t159;
            let t256 = f64x8::splat(1.0) / t44;
            let t257 = param_task_d * t183 * t256;
            let t259 = t160 * t257 + param_task_h0x * t183 + t237 + t254;
            let t264 = ((t3).select(f64x8::splat(0.0), -t7 * t167 * t161 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t259));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t264 + f64x8::splat(2.0) * t165;
            acc_vrho = tvrho0;
            let t270 = ((t37).select(t26 * t28 * t33 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t271 = t270 * t42;
            let t274 = ((t37).select(-t173 * t271 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t276 = t59 * t61;
            let t278 = ((t63).select(-t276 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t279 = t185 * t278;
            let t282 = t200 * t278;
            let t285 = t69 * t278;
            let t286 = t98 * t285;
            let t289 = t108 * t278;
            let t292 = f64x8::splat(29160.0) * t107 * t289 + f64x8::splat(324000.0) * t199 * t282 + f64x8::splat(120000.0) * t48 * t279 + f64x8::splat(97200.0) * t94 * t286;
            let t296 = -t292 * t127 + f64x8::splat(40.0) * t228 * t289;
            let t298 = t296 * t157 * t159;
            let t300 = f64x8::splat(2.0) * t146 + t143;
            let t304 = t300 * t155 - f64x8::splat(2.0) * t249 * t4;
            let t306 = t129 * t304 * t159;
            let t307 = param_task_d * t274;
            let t308 = t307 * t256;
            let t310 = t160 * t308 + param_task_h0x * t274 + t298 + t306;
            let t314 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t310));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t314;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t316 = t7 * t18;
            let t317 = t82 * t4;
            let t320 = f64x8::splat(1.0) / t46;
            let t324 = ((t63).select(t61 - t66 * t320 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t325 = t185 * t324;
            let t328 = t81 * t46;
            let t332 = t200 * t324;
            let t335 = t97 * v_tau;
            let t336 = t335 * t70;
            let t339 = t69 * t324;
            let t340 = t98 * t339;
            let t343 = t106 * t4;
            let t344 = t343 * t69;
            let t347 = t108 * t324;
            let t350 = f64x8::splat(324000.0) * t80 * t328 * t84 + f64x8::splat(29160.0) * t105 * t344 + f64x8::splat(29160.0) * t107 * t347 + f64x8::splat(324000.0) * t199 * t332 + f64x8::splat(120000.0) * t317 * t72 + f64x8::splat(120000.0) * t48 * t325 + f64x8::splat(97200.0) * t94 * t336 + f64x8::splat(97200.0) * t94 * t340;
            let t354 = f64x8::splat(10.0) * t4 * t69 + f64x8::splat(10.0) * t347;
            let t357 = -t350 * t127 + f64x8::splat(4.0) * t228 * t354;
            let t358 = t19 * t357;
            let t359 = t157 * t159;
            let t363 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t316 * t358 * t359));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t363;
            acc_vtau = tvtau0;
            let t366 = f64x8::splat(1.0) / t81;
            let t367 = t18 * t366;
            let t374 = t38 * t38;
            let t377 = param_task_c / t39 / t374;
            let t378 = t179 * t179;
            let t379 = t378 * t42;
            let t382 = t31 * t102;
            let t383 = f64x8::splat(1.0) / t382;
            let t387 = ((t37).select(f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t29 * t383, f64x8::splat(0.0)));
            let t391 = param_task_c * param_task_c;
            let t392 = ((t38).sqrt());
            let t395 = t391 / t392 / t374;
            let t399 = ((t37).select(f64x8::splat(5.0) / f64x8::splat(16.0) * t377 * t379 - t173 * t387 * t42 / f64x8::splat(4.0) - t395 * t379 / f64x8::splat(16.0), f64x8::splat(0.0)));
            let t401 = t54 * t70;
            let t402 = t191 * t191;
            let t403 = t401 * t402;
            let t407 = f64x8::splat(1.0) / t88;
            let t408 = t65 * t407;
            let t412 = ((t63).select(-f64x8::splat(2.0) * t186 + t408 * t61 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t413 = t185 * t412;
            let t416 = f64x8::splat(1.0) / t19;
            let t421 = t80 * t31;
            let t424 = t82 * t69;
            let t425 = t424 * t402;
            let t428 = t200 * t412;
            let t431 = t19 * v_rho;
            let t432 = t431 * t91;
            let t433 = t432 * t93;
            let t438 = t98 * t402;
            let t441 = t69 * t412;
            let t442 = t98 * t441;
            let t445 = t88 * t104;
            let t446 = t445 * t106;
            let t451 = t108 * t412;
            let t457 = f64x8::splat(120000.0) * t80 * t416 * t82 * t84 + f64x8::splat(247860.0) * t382 * t115 * t117 + f64x8::splat(29160.0) * t107 * t451 + f64x8::splat(583200.0) * t446 * t109 + f64x8::splat(648000.0) * t199 * t425 + f64x8::splat(324000.0) * t199 * t428 + f64x8::splat(1080000.0) * t421 * t201 + f64x8::splat(648000.0) * t206 * t210 + f64x8::splat(291600.0) * t214 * t217 + f64x8::splat(360000.0) * t48 * t403 + f64x8::splat(120000.0) * t48 * t413 + f64x8::splat(378000.0) * t433 * t99 + f64x8::splat(97200.0) * t94 * t438 + f64x8::splat(97200.0) * t94 * t442;
            let t459 = t224 * t227;
            let t463 = f64x8::splat(1.0) / t126 / t125;
            let t464 = t120 * t463;
            let t465 = t232 * t232;
            let t468 = t416 * t75;
            let t470 = f64x8::splat(10.0) * t468 + f64x8::splat(10.0) * t451;
            let t473 = -t457 * t127 + f64x8::splat(4.0) * t228 * t470 + f64x8::splat(8.0) * t459 * t232 - f64x8::splat(20.0) * t464 * t465;
            let t475 = t473 * t157 * t159;
            let t477 = t235 * t252 * t159;
            let t483 = t140 * t31;
            let t487 = f64x8::splat(3328.0) * t134 * t89 + f64x8::splat(320.0) / f64x8::splat(3.0) * t138 * t483 * v_sigma;
            let t489 = t245 * t248;
            let t492 = t154 * t154;
            let t493 = f64x8::splat(1.0) / t492;
            let t494 = t149 * t493;
            let t499 = t487 * t155 - f64x8::splat(128.0) * t489 * t121 + f64x8::splat(6144.0) * t494 * t92 - f64x8::splat(320.0) / f64x8::splat(3.0) * t249 * t229;
            let t501 = t129 * t499 * t159;
            let t504 = param_task_d * param_task_d;
            let t505 = t183 * t183;
            let t506 = t504 * t505;
            let t507 = t44 * t44;
            let t508 = f64x8::splat(1.0) / t507;
            let t509 = t506 * t508;
            let t511 = param_task_d * t399;
            let t512 = t511 * t256;
            let t515 = param_task_d * t505 * t508;
            let t517 = t160 * t509 + t160 * t512 - t160 * t515 + f64x8::splat(2.0) * t237 * t257 + f64x8::splat(2.0) * t254 * t257 + param_task_h0x * t399 + t475 + f64x8::splat(2.0) * t477 + t501;
            let t522 = ((t3).select(f64x8::splat(0.0), t7 * t367 * t161 / f64x8::splat(12.0) - t7 * t167 * t259 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t517));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t522 + f64x8::splat(4.0) * t264;
            acc_v2rho2 = tv2rho20;
            let t528 = t271 * t179;
            let t534 = ((t37).select(-t26 * t28 * t175 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t535 = t534 * t42;
            let t541 = ((t37).select(f64x8::splat(5.0) / f64x8::splat(16.0) * t377 * t528 - t173 * t535 / f64x8::splat(4.0) - t395 * t528 / f64x8::splat(16.0), f64x8::splat(0.0)));
            let t543 = t48 * t54;
            let t544 = t70 * t278;
            let t545 = t544 * t191;
            let t548 = t186 * t61;
            let t550 = ((t63).select(t548 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t551 = t185 * t550;
            let t556 = t278 * t191;
            let t557 = t424 * t556;
            let t560 = t200 * t550;
            let t565 = t98 * t556;
            let t568 = t69 * t550;
            let t569 = t98 * t568;
            let t574 = t108 * t550;
            let t577 = f64x8::splat(29160.0) * t107 * t574 + f64x8::splat(648000.0) * t199 * t557 + f64x8::splat(324000.0) * t199 * t560 + f64x8::splat(324000.0) * t206 * t286 + f64x8::splat(145800.0) * t214 * t289 + f64x8::splat(540000.0) * t421 * t282 + f64x8::splat(120000.0) * t48 * t551 + f64x8::splat(360000.0) * t543 * t545 + f64x8::splat(97200.0) * t94 * t565 + f64x8::splat(97200.0) * t94 * t569;
            let t579 = t292 * t227;
            let t584 = t464 * v_tau;
            let t585 = t4 * t278;
            let t586 = t585 * t232;
            let t591 = -t577 * t127 + f64x8::splat(40.0) * t228 * t574 + f64x8::splat(4.0) * t579 * t232 + f64x8::splat(40.0) * t459 * t289 - f64x8::splat(200.0) * t584 * t586;
            let t593 = t591 * t157 * t159;
            let t595 = t296 * t252 * t159;
            let t598 = t235 * t304 * t159;
            let t602 = t300 * t248;
            let t608 = t4 * t81 * t75;
            let t611 = f64x8::splat(64.0) * t138 * t241 * t155 - f64x8::splat(64.0) * t602 * t121 - f64x8::splat(2.0) * t489 * t4 + f64x8::splat(192.0) * t494 * t608;
            let t613 = t129 * t611 * t159;
            let t617 = t504 * t183;
            let t618 = t508 * t274;
            let t619 = t617 * t618;
            let t621 = param_task_d * t541;
            let t622 = t621 * t256;
            let t624 = t508 * t183;
            let t625 = t307 * t624;
            let t627 = t160 * t619 + t160 * t622 - t160 * t625 + t237 * t308 + t254 * t308 + t298 * t257 + t306 * t257 + param_task_h0x * t541 + t593 + t595 + t598 + t613;
            let t632 = ((t3).select(f64x8::splat(0.0), -t7 * t167 * t310 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t627));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t632 + f64x8::splat(2.0) * t314;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t635 = t166 * t357;
            let t641 = t70 * t324;
            let t642 = t641 * t191;
            let t648 = ((t63).select(-t276 + t187 * t320 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t649 = t185 * t648;
            let t656 = t46 * t70;
            let t657 = t656 * t191;
            let t662 = t324 * t191;
            let t663 = t424 * t662;
            let t666 = t200 * t648;
            let t671 = t335 * t209;
            let t676 = t98 * t662;
            let t679 = t69 * t648;
            let t680 = t98 * t679;
            let t685 = t343 * t191;
            let t690 = t108 * t648;
            let t693 = f64x8::splat(480000.0) * t317 * t192 + f64x8::splat(360000.0) * t543 * t642 + f64x8::splat(120000.0) * t48 * t649 + f64x8::splat(540000.0) * t80 * t31 * t46 * t84 + f64x8::splat(972000.0) * t199 * t657 + f64x8::splat(540000.0) * t421 * t332 + f64x8::splat(648000.0) * t199 * t663 + f64x8::splat(324000.0) * t199 * t666 + f64x8::splat(324000.0) * t206 * t336 + f64x8::splat(194400.0) * t94 * t671 + f64x8::splat(324000.0) * t206 * t340 + f64x8::splat(97200.0) * t94 * t676 + f64x8::splat(97200.0) * t94 * t680 + f64x8::splat(145800.0) * t213 * t344 + f64x8::splat(29160.0) * t105 * t685 + f64x8::splat(145800.0) * t214 * t347 + f64x8::splat(29160.0) * t107 * t690;
            let t695 = t350 * t227;
            let t700 = t354 * t232;
            let t705 = f64x8::splat(10.0) * t4 * t191 + f64x8::splat(10.0) * t690;
            let t708 = -t693 * t127 + f64x8::splat(4.0) * t228 * t705 + f64x8::splat(4.0) * t695 * t232 + f64x8::splat(4.0) * t459 * t354 - f64x8::splat(20.0) * t464 * t700;
            let t709 = t19 * t708;
            let t713 = t252 * t159;
            let t718 = t7 * t20 * t357;
            let t719 = t359 * t257;
            let t723 = ((t3).select(f64x8::splat(0.0), -t316 * t635 * t359 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t316 * t709 * t359 - f64x8::splat(3.0) / f64x8::splat(8.0) * t316 * t358 * t713 - f64x8::splat(3.0) / f64x8::splat(8.0) * t718 * t719));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t723 + f64x8::splat(2.0) * t363;
            acc_v2rhotau = tv2rhotau0;
            let t726 = t270 * t270;
            let t727 = t726 * t42;
            let t730 = ((t37).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t731 = t730 * t42;
            let t733 = t173 * t731 / f64x8::splat(4.0);
            let t737 = ((t37).select(f64x8::splat(5.0) / f64x8::splat(16.0) * t377 * t727 - t733 - t395 * t727 / f64x8::splat(16.0), f64x8::splat(0.0)));
            let t739 = t278 * t278;
            let t740 = t401 * t739;
            let t743 = ((t63).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t744 = t185 * t743;
            let t746 = f64x8::splat(120000.0) * t48 * t744;
            let t747 = t424 * t739;
            let t750 = t200 * t743;
            let t752 = f64x8::splat(324000.0) * t199 * t750;
            let t753 = t98 * t739;
            let t756 = t69 * t743;
            let t757 = t98 * t756;
            let t759 = f64x8::splat(97200.0) * t94 * t757;
            let t760 = t108 * t743;
            let t762 = f64x8::splat(29160.0) * t107 * t760;
            let t763 = f64x8::splat(648000.0) * t199 * t747 + f64x8::splat(360000.0) * t48 * t740 + f64x8::splat(97200.0) * t94 * t753 + t746 + t752 + t759 + t762;
            let t767 = t46 * t93;
            let t768 = t767 * t739;
            let t772 = f64x8::splat(40.0) * t228 * t760;
            let t773 = -t763 * t127 + f64x8::splat(80.0) * t579 * t289 - f64x8::splat(2000.0) * t464 * t768 + t772;
            let t775 = t773 * t157 * t159;
            let t777 = t296 * t304 * t159;
            let t781 = t93 * t145;
            let t788 = f64x8::splat(2.0) * t781 * t155 - f64x8::splat(4.0) * t602 * t4 + f64x8::splat(6.0) * t494 * t93;
            let t790 = t129 * t788 * t159;
            let t793 = t274 * t274;
            let t794 = t504 * t793;
            let t795 = t794 * t508;
            let t797 = param_task_d * t737;
            let t798 = t797 * t256;
            let t800 = param_task_d * t793;
            let t801 = t800 * t508;
            let t803 = t160 * t795 + t160 * t798 - t160 * t801 + f64x8::splat(2.0) * t298 * t308 + f64x8::splat(2.0) * t306 * t308 + param_task_h0x * t737 + t775 + f64x8::splat(2.0) * t777 + t790;
            let t807 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t803));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t807;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t811 = t641 * t278;
            let t814 = t59 * t320;
            let t816 = ((t63).select(t814 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t817 = t185 * t816;
            let t820 = t656 * t278;
            let t823 = t324 * t278;
            let t824 = t424 * t823;
            let t827 = t200 * t816;
            let t830 = t335 * t285;
            let t833 = t98 * t823;
            let t836 = t69 * t816;
            let t837 = t98 * t836;
            let t840 = t343 * t278;
            let t843 = t108 * t816;
            let t846 = f64x8::splat(29160.0) * t105 * t840 + f64x8::splat(29160.0) * t107 * t843 + f64x8::splat(972000.0) * t199 * t820 + f64x8::splat(648000.0) * t199 * t824 + f64x8::splat(324000.0) * t199 * t827 + f64x8::splat(480000.0) * t317 * t279 + f64x8::splat(120000.0) * t48 * t817 + f64x8::splat(360000.0) * t543 * t811 + f64x8::splat(194400.0) * t94 * t830 + f64x8::splat(97200.0) * t94 * t833 + f64x8::splat(97200.0) * t94 * t837;
            let t852 = t464 * t354;
            let t856 = f64x8::splat(10.0) * t585 + f64x8::splat(10.0) * t843;
            let t859 = -t846 * t127 + f64x8::splat(4.0) * t228 * t856 + f64x8::splat(40.0) * t695 * t289 - f64x8::splat(200.0) * t852 * t289 + f64x8::splat(4.0) * t579 * t354;
            let t860 = t19 * t859;
            let t863 = t304 * t159;
            let t866 = t359 * t308;
            let t870 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t316 * t358 * t863 - f64x8::splat(3.0) / f64x8::splat(8.0) * t316 * t860 * t359 - f64x8::splat(3.0) / f64x8::splat(8.0) * t718 * t866));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t870;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t872 = t46 * t4;
            let t877 = t324 * t324;
            let t878 = t401 * t877;
            let t882 = f64x8::splat(1.0) / t82;
            let t886 = ((t63).select(-f64x8::splat(2.0) * t320 + t66 * t882 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t887 = t185 * t886;
            let t894 = t656 * t324;
            let t897 = t424 * t877;
            let t900 = t200 * t886;
            let t903 = t93 * t97;
            let t904 = t903 * t70;
            let t907 = t335 * t339;
            let t910 = t98 * t877;
            let t913 = t69 * t886;
            let t914 = t98 * t913;
            let t917 = t343 * t324;
            let t920 = t108 * t886;
            let t923 = f64x8::splat(648000.0) * t80 * t81 * v_tau * t84 + f64x8::splat(58320.0) * t105 * t917 + f64x8::splat(29160.0) * t107 * t920 + f64x8::splat(1944000.0) * t199 * t894 + f64x8::splat(648000.0) * t199 * t897 + f64x8::splat(324000.0) * t199 * t900 + f64x8::splat(960000.0) * t317 * t325 + f64x8::splat(360000.0) * t48 * t878 + f64x8::splat(120000.0) * t48 * t887 + f64x8::splat(360000.0) * t872 * t72 + f64x8::splat(97200.0) * t92 * t904 + f64x8::splat(388800.0) * t94 * t907 + f64x8::splat(97200.0) * t94 * t910 + f64x8::splat(97200.0) * t94 * t914;
            let t927 = t354 * t354;
            let t933 = f64x8::splat(20.0) * t4 * t324 + f64x8::splat(10.0) * t920;
            let t936 = -t923 * t127 + f64x8::splat(4.0) * t228 * t933 + f64x8::splat(8.0) * t695 * t354 - f64x8::splat(20.0) * t464 * t927;
            let t937 = t19 * t936;
            let t941 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t316 * t937 * t359));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t941;
            acc_v2tau2 = tv2tau20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rholapl.into(); v2rholapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhotau.into(); v2rhotau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmalapl.into(); v2sigmalapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmatau.into(); v2sigmatau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapl2.into(); v2lapl2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapltau.into(); v2lapltau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2tau2.into(); v2tau2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
