//! LDA_K_GDS08_WORKER fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_gds08_worker.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_gds08_worker_fxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 <= dens_threshold;
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = 1.0 - t5 <= zeta_threshold;
        let t11 = -t8;
        let t12 = piecewise5(t7, t8, t10, t11, t5);
        let t13 = 1.0 + t12;
        let t16 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t19 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t20 = piecewise5(t16, t8, t19, t11, t5);
        let t21 = 1.0 + t20;
        let t23 = rmath::ln(t21 * t3);
        let t25 = t23 * t23;
        let t27 = t23 * param_B + t25 * param_C + param_A;
        let t30 = piecewise3(t1, 0.0, t13 * t27 / 2.0);
        let t31 = rho1 <= dens_threshold;
        let t32 = piecewise5(t10, t8, t7, t11, -t5);
        let t33 = 1.0 + t32;
        let t34 = -t2;
        let t36 = piecewise5(t19, t8, t16, t11, t34 * t4);
        let t37 = 1.0 + t36;
        let t39 = rmath::ln(t37 * t3);
        let t41 = t39 * t39;
        let t43 = t39 * param_B + t41 * param_C + param_A;
        let t46 = piecewise3(t31, 0.0, t33 * t43 / 2.0);
        let tzk0 = t30 + t46;
        zk[ip] += tzk0;
        let t47 = t3 * t3;
        let t48 = 1.0 / t47;
        let t49 = t2 * t48;
        let t50 = t4 - t49;
        let t51 = piecewise5(t7, 0.0, t10, 0.0, t50);
        let t53 = piecewise5(t16, 0.0, t19, 0.0, t50);
        let t55 = t3 * t53 + t20 + 1.0;
        let t56 = param_B * t55;
        let t57 = 1.0 / t21;
        let t58 = t57 * t4;
        let t60 = param_C * t23;
        let t61 = t55 * t57;
        let t65 = 2.0 * t4 * t60 * t61 + t56 * t58;
        let t69 = piecewise3(t1, 0.0, t13 * t65 / 2.0 + t51 * t27 / 2.0);
        let t71 = piecewise5(t10, 0.0, t7, 0.0, -t50);
        let t73 = t34 * t48;
        let t75 = piecewise5(t19, 0.0, t16, 0.0, -t4 - t73);
        let t77 = t3 * t75 + t36 + 1.0;
        let t78 = param_B * t77;
        let t79 = 1.0 / t37;
        let t80 = t79 * t4;
        let t82 = param_C * t39;
        let t83 = t77 * t79;
        let t87 = 2.0 * t4 * t82 * t83 + t78 * t80;
        let t91 = piecewise3(t31, 0.0, t33 * t87 / 2.0 + t71 * t43 / 2.0);
        let tvrho0 = t30 + t46 + t3 * (t69 + t91);
        vrho[ip * 2] += tvrho0;
        let t94 = -t4 - t49;
        let t95 = piecewise5(t7, 0.0, t10, 0.0, t94);
        let t97 = piecewise5(t16, 0.0, t19, 0.0, t94);
        let t99 = t3 * t97 + t20 + 1.0;
        let t100 = param_B * t99;
        let t102 = t99 * t57;
        let t106 = 2.0 * t102 * t4 * t60 + t100 * t58;
        let t110 = piecewise3(t1, 0.0, t13 * t106 / 2.0 + t95 * t27 / 2.0);
        let t112 = piecewise5(t10, 0.0, t7, 0.0, -t94);
        let t115 = piecewise5(t19, 0.0, t16, 0.0, t4 - t73);
        let t117 = t115 * t3 + t36 + 1.0;
        let t118 = param_B * t117;
        let t120 = t117 * t79;
        let t124 = 2.0 * t120 * t4 * t82 + t118 * t80;
        let t128 = piecewise3(t31, 0.0, t112 * t43 / 2.0 + t33 * t124 / 2.0);
        let tvrho1 = t30 + t46 + t3 * (t110 + t128);
        vrho[ip * 2 + 1] += tvrho1;
        let t134 = 1.0 / t47 / t3;
        let t135 = t2 * t134;
        let t137 = -2.0 * t48 + 2.0 * t135;
        let t138 = piecewise5(t7, 0.0, t10, 0.0, t137);
        let t142 = piecewise5(t16, 0.0, t19, 0.0, t137);
        let t145 = t142 * t3 + 2.0 * t53;
        let t146 = param_B * t145;
        let t148 = t21 * t21;
        let t149 = 1.0 / t148;
        let t150 = t149 * t4;
        let t151 = t150 * t53;
        let t153 = t57 * t48;
        let t155 = t55 * t55;
        let t156 = param_C * t155;
        let t157 = t149 * t48;
        let t160 = t145 * t57;
        let t164 = t60 * t55;
        let t170 = 2.0 * t160 * t4 * t60 - 2.0 * t48 * t60 * t61 + t146 * t58 - 2.0 * t151 * t164 - t151 * t56 - t153 * t56 + 2.0 * t156 * t157;
        let t174 = piecewise3(t1, 0.0, t138 * t27 / 2.0 + t51 * t65 + t13 * t170 / 2.0);
        let t176 = piecewise5(t10, 0.0, t7, 0.0, -t137);
        let t180 = t34 * t134;
        let t183 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t48 + 2.0 * t180);
        let t186 = t183 * t3 + 2.0 * t75;
        let t187 = param_B * t186;
        let t189 = t37 * t37;
        let t190 = 1.0 / t189;
        let t191 = t190 * t4;
        let t192 = t191 * t75;
        let t194 = t79 * t48;
        let t196 = t77 * t77;
        let t197 = param_C * t196;
        let t198 = t190 * t48;
        let t201 = t186 * t79;
        let t205 = t82 * t77;
        let t211 = 2.0 * t201 * t4 * t82 - 2.0 * t48 * t82 * t83 + t187 * t80 - 2.0 * t192 * t205 - t192 * t78 - t194 * t78 + 2.0 * t197 * t198;
        let t215 = piecewise3(t31, 0.0, t176 * t43 / 2.0 + t71 * t87 + t33 * t211 / 2.0);
        let tv2rho20 = 2.0 * t69 + 2.0 * t91 + t3 * (t174 + t215);
        v2rho2[ip * 3] += tv2rho20;
        let t218 = 2.0 * t135;
        let t219 = piecewise5(t7, 0.0, t10, 0.0, t218);
        let t223 = piecewise5(t16, 0.0, t19, 0.0, t218);
        let t225 = t223 * t3 + t53 + t97;
        let t226 = param_B * t225;
        let t229 = t100 * t153;
        let t230 = param_C * t55;
        let t231 = t157 * t99;
        let t234 = t225 * t57;
        let t238 = t60 * t99;
        let t243 = 2.0 * t60 * t102 * t48;
        let t244 = 2.0 * t234 * t4 * t60 - t100 * t151 - 2.0 * t151 * t238 + t226 * t58 + 2.0 * t230 * t231 - t229 - t243;
        let t248 = piecewise3(t1, 0.0, t51 * t106 / 2.0 + t13 * t244 / 2.0 + t219 * t27 / 2.0 + t95 * t65 / 2.0);
        let t249 = piecewise5(t10, 0.0, t7, 0.0, -t218);
        let t254 = piecewise5(t19, 0.0, t16, 0.0, 2.0 * t180);
        let t256 = t254 * t3 + t115 + t75;
        let t257 = param_B * t256;
        let t260 = t118 * t194;
        let t261 = param_C * t77;
        let t262 = t198 * t117;
        let t265 = t256 * t79;
        let t269 = t82 * t117;
        let t274 = 2.0 * t82 * t120 * t48;
        let t275 = 2.0 * t265 * t4 * t82 - t118 * t192 - 2.0 * t192 * t269 + t257 * t80 + 2.0 * t261 * t262 - t260 - t274;
        let t279 = piecewise3(t31, 0.0, t112 * t87 / 2.0 + t71 * t124 / 2.0 + t249 * t43 / 2.0 + t33 * t275 / 2.0);
        let tv2rho21 = t69 + t91 + t110 + t128 + t3 * (t248 + t279);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t285 = 2.0 * t48 + 2.0 * t135;
        let t286 = piecewise5(t7, 0.0, t10, 0.0, t285);
        let t290 = piecewise5(t16, 0.0, t19, 0.0, t285);
        let t293 = t290 * t3 + 2.0 * t97;
        let t294 = param_B * t293;
        let t296 = t150 * t97;
        let t298 = t99 * t99;
        let t299 = param_C * t298;
        let t302 = t293 * t57;
        let t308 = 2.0 * t302 * t4 * t60 - t100 * t296 + 2.0 * t157 * t299 - 2.0 * t238 * t296 + t294 * t58 - t229 - t243;
        let t312 = piecewise3(t1, 0.0, t286 * t27 / 2.0 + t95 * t106 + t13 * t308 / 2.0);
        let t314 = piecewise5(t10, 0.0, t7, 0.0, -t285);
        let t320 = piecewise5(t19, 0.0, t16, 0.0, -2.0 * t48 + 2.0 * t180);
        let t323 = t3 * t320 + 2.0 * t115;
        let t324 = param_B * t323;
        let t326 = t191 * t115;
        let t328 = t117 * t117;
        let t329 = param_C * t328;
        let t332 = t323 * t79;
        let t338 = 2.0 * t332 * t4 * t82 - t118 * t326 + 2.0 * t198 * t329 - 2.0 * t269 * t326 + t324 * t80 - t260 - t274;
        let t342 = piecewise3(t31, 0.0, t314 * t43 / 2.0 + t112 * t124 + t33 * t338 / 2.0);
        let tv2rho22 = 2.0 * t110 + 2.0 * t128 + t3 * (t312 + t342);
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
