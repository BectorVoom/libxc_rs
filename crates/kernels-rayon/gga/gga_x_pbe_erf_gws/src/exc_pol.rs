//! GGA_X_PBE_ERF_GWS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe_erf_gws.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbe_erf_gws_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_ax: f64,
    param_b_PBE: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRTPI;
        let t3 = 1.0 / t2;
        let t4 = param_hyb_omega_0 * param_hyb_omega_0;
        let t5 = param_ax * t4;
        let t6 = M_CBRT3;
        let t7 = t5 * t6;
        let t8 = t2 * M_PI;
        let t9 = 1.0 / t8;
        let t10 = 2.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = M_CBRT2;
        let t13 = piecewise3(t10, t11, t12);
        let t14 = t13 * t13;
        let t15 = 1.0 / t14;
        let t16 = t9 * t15;
        let t17 = pow_1_3(rho0);
        let t18 = t17 * t17;
        let t19 = 1.0 / t18;
        let t23 = rmath::exp(-t7 * t16 * t19 / 12.0);
        let t25 = param_b_PBE * t23 * sigma0;
        let t26 = t6 * t12;
        let t27 = param_kappa + 1.0;
        let t28 = t6 * t6;
        let t29 = t2 * t2;
        let t31 = t28 / t29;
        let t32 = 1.0 / t17;
        let t34 = 1.0 / t13;
        let t37 = t31 * param_hyb_omega_0 * t32 * t34 / 6.0;
        let t38 = t37 < 0.05;
        let t39 = t14 * t14;
        let t40 = M_PI * M_PI;
        let t41 = t29 * t40;
        let t42 = t39 * t41;
        let t43 = t17 * rho0;
        let t44 = t42 * t43;
        let t46 = t14 * t8;
        let t49 = t46 * t6 * t18 * t4;
        let t51 = 7.0 * t44 - 6.0 * t49;
        let t52 = t14 * t13;
        let t53 = 1.0 / param_hyb_omega_0;
        let t54 = t53 * t6;
        let t55 = t29 * t13;
        let t58 = rmath::erf(t54 * t55 * t17);
        let t59 = t52 * t58;
        let t60 = rmath::sqrt(M_PI);
        let t61 = t60 * t40;
        let t69 = t4 * t4;
        let t71 = 6.0 * t28 * t69;
        let t72 = -36.0 * t59 * t61 * t28 * rho0 * param_hyb_omega_0 + 81.0 * t44 + 54.0 * t49 - t71;
        let t73 = 1.0 / t72;
        let t75 = 10000000000.0 < t37;
        let t76 = t40 * t40;
        let t77 = rho0 * rho0;
        let t79 = t39 * t14;
        let t82 = t8 * t28;
        let t87 = t41 * t6;
        let t93 = t69 * t4;
        let t94 = 1.0 / t93;
        let t98 = 1.0 / t4;
        let t99 = t98 * t28;
        let t101 = t99 * t46 * t18;
        let t102 = rmath::exp(t101);
        let t103 = t102 * t6;
        let t104 = t103 * t14;
        let t106 = t8 * t18 * t4;
        let t110 = t102 * t28;
        let t114 = (7.0 * t104 * t106 - 6.0 * t110 * t69 + 6.0 * t44 + 11.0 * t49 + t71) * t8;
        let t116 = t14 * t6;
        let t117 = t39 * t102;
        let t118 = t41 * t28;
        let t123 = t52 * t102 * t58;
        let t124 = t61 * t6;
        let t129 = t14 * t102;
        let t138 = 2.0 * t69 * t6;
        let t139 = 12.0 * t123 * t124 * rho0 * param_hyb_omega_0 - 9.0 * t117 * t118 * t43 + 12.0 * t46 * t18 * t4 + 2.0 * t103 * t69 - 18.0 * t129 * t106 - t138;
        let t142 = t116 * t98 / t139;
        let t145 = piecewise5(t38, t51 * t73, t75, (2800.0 * t82 * t18 * t69 * t14 - 140.0 * t87 * t43 * t4 * t39 - 1863.0 * t76 * t77 * t79) * t94 / 50400.0, -t114 * t18 * t142 / 9.0);
        let t146 = t27 * t145;
        let t150 = t18 * t77;
        let t152 = param_kappa * t150 * t8;
        let t153 = 27.0 / 56.0 * t25 * t26 * t146 + t152;
        let t154 = t3 * t153;
        let t155 = t154 * t6;
        let t158 = piecewise3(t10, t11 * zeta_threshold, 2.0 * t12);
        let t159 = t158 * t43;
        let t160 = 1.35 <= t37;
        let t161 = 1.35 < t37;
        let t162 = piecewise3(t161, t37, 1.35);
        let t163 = t162 * t162;
        let t164 = t163 * t163;
        let t165 = t164 * t163;
        let t166 = t164 * t164;
        let t169 = t166 * t164;
        let t171 = t166 * t163;
        let t177 = 24088884019200.0 * t166 * t165 + 19448.0 * t163 - 807840.0 * t164 + 30551040.0 * t165 - 1045524480.0 * t166 - 903333150720.0 * t169 + 32261898240.0 * t171 - 429.0;
        let t178 = t166 * t166;
        let t179 = 1.0 / t178;
        let t182 = piecewise3(t161, 1.35, t37);
        let t183 = t182 * t182;
        let t184 = t183 * t183;
        let t187 = 32.0 * t184 - 16.0 * t183;
        let t190 = rmath::exp(-1.0 / t183 / 4.0);
        let t194 = 1.0 / t182;
        let t196 = rmath::erf(t194 / 2.0);
        let t197 = t60 * t196;
        let t202 = piecewise3(t160, t177 * t179 / 867199824691200.0, t187 * t190 / 3.0 - 32.0 / 3.0 * t184 - 8.0 / 3.0 * t197 * t182 + 8.0 * t183 + 1.0);
        let t203 = param_b_PBE * t145;
        let t206 = sigma0 * t12 * t6;
        let t210 = 216.0 * t203 * t23 * t206 + 448.0 * t152;
        let t211 = 1.0 / t210;
        let t212 = t202 * t211;
        let t213 = t159 * t212;
        let t216 = piecewise3(t1, 0.0, -168.0 * t155 * t213);
        let t217 = rho1 <= dens_threshold;
        let t218 = pow_1_3(rho1);
        let t219 = t218 * t218;
        let t220 = 1.0 / t219;
        let t224 = rmath::exp(-t7 * t16 * t220 / 12.0);
        let t226 = param_b_PBE * t224 * sigma2;
        let t227 = 1.0 / t218;
        let t231 = t31 * param_hyb_omega_0 * t227 * t34 / 6.0;
        let t232 = t231 < 0.05;
        let t233 = t218 * rho1;
        let t234 = t42 * t233;
        let t238 = t46 * t6 * t219 * t4;
        let t240 = 7.0 * t234 - 6.0 * t238;
        let t243 = rmath::erf(t54 * t55 * t218);
        let t244 = t52 * t243;
        let t252 = -36.0 * t244 * t61 * t28 * rho1 * param_hyb_omega_0 + 81.0 * t234 + 54.0 * t238 - t71;
        let t253 = 1.0 / t252;
        let t255 = 10000000000.0 < t231;
        let t256 = rho1 * rho1;
        let t273 = t99 * t46 * t219;
        let t274 = rmath::exp(t273);
        let t275 = t274 * t219;
        let t277 = t46 * t4;
        let t281 = t274 * t28;
        let t285 = (7.0 * t275 * t6 * t277 - 6.0 * t281 * t69 + 6.0 * t234 + 11.0 * t238 + t71) * t8;
        let t287 = t39 * t274;
        let t292 = t52 * t274 * t243;
        let t297 = t14 * t274;
        let t305 = t274 * t6;
        let t308 = 12.0 * t292 * t124 * rho1 * param_hyb_omega_0 - 18.0 * t297 * t8 * t219 * t4 - 9.0 * t287 * t118 * t233 + 12.0 * t46 * t219 * t4 + 2.0 * t305 * t69 - t138;
        let t311 = t116 * t98 / t308;
        let t314 = piecewise5(t232, t240 * t253, t255, (2800.0 * t82 * t219 * t69 * t14 - 140.0 * t87 * t233 * t4 * t39 - 1863.0 * t76 * t256 * t79) * t94 / 50400.0, -t285 * t219 * t311 / 9.0);
        let t315 = t27 * t314;
        let t319 = t219 * t256;
        let t321 = param_kappa * t319 * t8;
        let t322 = 27.0 / 56.0 * t226 * t26 * t315 + t321;
        let t323 = t3 * t322;
        let t324 = t323 * t6;
        let t325 = t158 * t233;
        let t326 = 1.35 <= t231;
        let t327 = 1.35 < t231;
        let t328 = piecewise3(t327, t231, 1.35);
        let t329 = t328 * t328;
        let t330 = t329 * t329;
        let t331 = t330 * t329;
        let t332 = t330 * t330;
        let t335 = t332 * t330;
        let t337 = t332 * t329;
        let t343 = 24088884019200.0 * t332 * t331 + 19448.0 * t329 - 807840.0 * t330 + 30551040.0 * t331 - 1045524480.0 * t332 - 903333150720.0 * t335 + 32261898240.0 * t337 - 429.0;
        let t344 = t332 * t332;
        let t345 = 1.0 / t344;
        let t348 = piecewise3(t327, 1.35, t231);
        let t349 = t348 * t348;
        let t350 = t349 * t349;
        let t353 = 32.0 * t350 - 16.0 * t349;
        let t356 = rmath::exp(-1.0 / t349 / 4.0);
        let t360 = 1.0 / t348;
        let t362 = rmath::erf(t360 / 2.0);
        let t368 = piecewise3(t326, t343 * t345 / 867199824691200.0, t353 * t356 / 3.0 - 32.0 / 3.0 * t350 - 8.0 / 3.0 * t362 * t348 * t60 + 8.0 * t349 + 1.0);
        let t369 = param_b_PBE * t314;
        let t372 = sigma2 * t12 * t6;
        let t376 = 216.0 * t369 * t224 * t372 + 448.0 * t321;
        let t377 = 1.0 / t376;
        let t378 = t368 * t377;
        let t379 = t325 * t378;
        let t382 = piecewise3(t217, 0.0, -168.0 * t324 * t379);
        let tzk0 = (t216 + t382) / (rho0 + rho1);
        zk[ip] += tzk0;
    }
}
