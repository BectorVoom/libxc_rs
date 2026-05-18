//! GGA_C_BMK vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_bmk.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_bmk_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_c_ab_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = piecewise3::<f64>(t3, zeta_threshold, 1.0);
        let t6 = M_CBRT3;
        let t7 = 1.0 / M_PI;
        let t8 = pow_1_3::<f64>(t7);
        let t9 = t6 * t8;
        let t10 = M_CBRT4;
        let t11 = t10 * t10;
        let t12 = t9 * t11;
        let t13 = pow_1_3::<f64>(rho[ip]);
        let t14 = 1.0 / t13;
        let t15 = M_CBRT2;
        let t17 = pow_1_3::<f64>(zeta_threshold);
        let t19 = piecewise3::<f64>(t3, 1.0 / t17, 1.0);
        let t21 = t12 * t14 * t15 * t19;
        let t23 = 1.0 + 0.53425e-1 * t21;
        let t24 = f64::sqrt(t21);
        let t27 = pow_3_2::<f64>(t21);
        let t29 = t6 * t6;
        let t30 = t8 * t8;
        let t31 = t29 * t30;
        let t32 = t31 * t10;
        let t33 = t13 * t13;
        let t34 = 1.0 / t33;
        let t35 = t15 * t15;
        let t37 = t19 * t19;
        let t39 = t32 * t34 * t35 * t37;
        let t41 = 0.379785e1 * t24 + 0.8969e0 * t21 + 0.204775e0 * t27 + 0.123235e0 * t39;
        let t44 = 1.0 + 0.16081824322151104822e2 / t41;
        let t45 = f64::ln(t44);
        let t47 = 0.62182e-1 * t23 * t45;
        let t49 = t17 * zeta_threshold;
        let t51 = piecewise3::<f64>(2.0 <= zeta_threshold, t49, 2.0 * t15);
        let t53 = piecewise3::<f64>(0.0 <= zeta_threshold, t49, 0.0);
        let t57 = 1.0 / (2.0 * t15 - 2.0);
        let t58 = (t51 + t53 - 2.0) * t57;
        let t60 = 1.0 + 0.5137e-1 * t21;
        let t65 = 0.705945e1 * t24 + 0.1549425e1 * t21 + 0.420775e0 * t27 + 0.1562925e0 * t39;
        let t68 = 1.0 + 0.32164683177870697974e2 / t65;
        let t69 = f64::ln(t68);
        let t73 = 1.0 + 0.278125e-1 * t21;
        let t78 = 0.51785e1 * t24 + 0.905775e0 * t21 + 0.1100325e0 * t27 + 0.1241775e0 * t39;
        let t81 = 1.0 + 0.29608574643216675549e2 / t78;
        let t82 = f64::ln(t81);
        let t83 = t73 * t82;
        let t92 = piecewise3::<f64>(t4, 0.0, t5 * (-t47 + t58 * (-0.3109e-1 * t60 * t69 + t47 - 0.19751789702565206229e-1 * t83) + 0.19751789702565206229e-1 * t58 * t83) / 2.0);
        let t94 = param_c_ss_1;
        let t95 = t94 * sigma[ip];
        let t96 = rho[ip] * rho[ip];
        let t98 = 1.0 / t33 / t96;
        let t99 = t35 * t98;
        let t101 = sigma[ip] * t35 * t98;
        let t103 = 1.0 + 0.2e0 * t101;
        let t104 = 1.0 / t103;
        let t108 = param_c_ss_2;
        let t109 = sigma[ip] * sigma[ip];
        let t110 = t108 * t109;
        let t111 = t96 * t96;
        let t112 = t111 * rho[ip];
        let t114 = 1.0 / t13 / t112;
        let t115 = t15 * t114;
        let t116 = t103 * t103;
        let t117 = 1.0 / t116;
        let t118 = t115 * t117;
        let t121 = param_c_ss_3;
        let t122 = t109 * sigma[ip];
        let t123 = t121 * t122;
        let t124 = t111 * t111;
        let t125 = 1.0 / t124;
        let t126 = t116 * t103;
        let t127 = 1.0 / t126;
        let t128 = t125 * t127;
        let t131 = param_c_ss_4;
        let t132 = t109 * t109;
        let t133 = t131 * t132;
        let t134 = t124 * t96;
        let t136 = 1.0 / t33 / t134;
        let t137 = t35 * t136;
        let t138 = t116 * t116;
        let t139 = 1.0 / t138;
        let t140 = t137 * t139;
        let t143 = param_c_ss_0 + 0.2e0 * t95 * t99 * t104 + 0.8e-1 * t110 * t118 + 0.32e-1 * t123 * t128 + 0.64e-2 * t133 * t140;
        let t145 = 2.0 * t92 * t143;
        let t147 = t9 * t11 * t14;
        let t149 = 1.0 + 0.53425e-1 * t147;
        let t150 = f64::sqrt(t147);
        let t153 = pow_3_2::<f64>(t147);
        let t156 = t31 * t10 * t34;
        let t158 = 0.379785e1 * t150 + 0.8969e0 * t147 + 0.204775e0 * t153 + 0.123235e0 * t156;
        let t161 = 1.0 + 0.16081824322151104822e2 / t158;
        let t162 = f64::ln(t161);
        let t165 = piecewise3::<f64>(t3, t49, 1.0);
        let t168 = (2.0 * t165 - 2.0) * t57;
        let t170 = 1.0 + 0.278125e-1 * t147;
        let t175 = 0.51785e1 * t150 + 0.905775e0 * t147 + 0.1100325e0 * t153 + 0.1241775e0 * t156;
        let t178 = 1.0 + 0.29608574643216675549e2 / t175;
        let t179 = f64::ln(t178);
        let t184 = -0.62182e-1 * t149 * t162 + 0.19751789702565206229e-1 * t168 * t170 * t179 - 2.0 * t92;
        let t186 = param_c_ab_1;
        let t187 = t186 * sigma[ip];
        let t189 = 1.0 + 0.6e-2 * t101;
        let t190 = 1.0 / t189;
        let t194 = param_c_ab_2;
        let t195 = t194 * t109;
        let t196 = t189 * t189;
        let t197 = 1.0 / t196;
        let t198 = t115 * t197;
        let t201 = param_c_ab_3;
        let t202 = t201 * t122;
        let t203 = t196 * t189;
        let t204 = 1.0 / t203;
        let t205 = t125 * t204;
        let t208 = param_c_ab_4;
        let t209 = t208 * t132;
        let t210 = t196 * t196;
        let t211 = 1.0 / t210;
        let t212 = t137 * t211;
        let t215 = param_c_ab_0 + 0.6e-2 * t187 * t99 * t190 + 0.72e-4 * t195 * t198 + 0.864e-6 * t202 * t205 + 0.5184e-8 * t209 * t212;
        let t216 = t184 * t215;
        let tzk0 = t145 + t216;
        zk[ip] += tzk0;
        let t218 = 1.0 / t13 / rho[ip];
        let t219 = t218 * t15;
        let t220 = t19 * t45;
        let t223 = 0.11073577833333333333e-2 * t12 * t219 * t220;
        let t224 = t41 * t41;
        let t225 = 1.0 / t224;
        let t226 = t23 * t225;
        let t229 = 1.0 / t24 * t6 * t8;
        let t230 = t11 * t218;
        let t231 = t15 * t19;
        let t232 = t230 * t231;
        let t233 = t229 * t232;
        let t235 = t219 * t19;
        let t236 = t12 * t235;
        let t238 = f64::sqrt(t21);
        let t240 = t238 * t6 * t8;
        let t241 = t240 * t232;
        let t244 = 1.0 / t33 / rho[ip];
        let t247 = t32 * t244 * t35 * t37;
        let t249 = -0.632975e0 * t233 - 0.29896666666666666667e0 * t236 - 0.1023875e0 * t241 - 0.82156666666666666667e-1 * t247;
        let t250 = 1.0 / t44;
        let t251 = t249 * t250;
        let t253 = 1.0 * t226 * t251;
        let t254 = t19 * t69;
        let t258 = t65 * t65;
        let t259 = 1.0 / t258;
        let t260 = t60 * t259;
        let t265 = -0.1176575e1 * t233 - 0.516475e0 * t236 - 0.2103875e0 * t241 - 0.104195e0 * t247;
        let t266 = 1.0 / t68;
        let t267 = t265 * t266;
        let t270 = t19 * t82;
        let t274 = t78 * t78;
        let t275 = 1.0 / t274;
        let t276 = t73 * t275;
        let t281 = -0.86308333333333333334e0 * t233 - 0.301925e0 * t236 - 0.5501625e-1 * t241 - 0.82785e-1 * t247;
        let t282 = 1.0 / t81;
        let t283 = t281 * t282;
        let t288 = t58 * t9;
        let t289 = t231 * t82;
        let t293 = t58 * t73;
        let t295 = t275 * t281 * t282;
        let t301 = piecewise3::<f64>(t4, 0.0, t5 * (t223 + t253 + t58 * (0.53236443333333333332e-3 * t12 * t219 * t254 + 1.0 * t260 * t267 - t223 - t253 + 0.18311555036753159941e-3 * t12 * t219 * t270 + 0.58482233974552040708e0 * t276 * t283) - 0.18311555036753159941e-3 * t288 * t230 * t289 - 0.58482233974552040708e0 * t293 * t295) / 2.0);
        let t302 = t301 * t143;
        let t304 = t96 * rho[ip];
        let t306 = 1.0 / t33 / t304;
        let t307 = t35 * t306;
        let t311 = t94 * t109;
        let t312 = t111 * t96;
        let t314 = 1.0 / t13 / t312;
        let t315 = t15 * t314;
        let t316 = t315 * t117;
        let t321 = t108 * t122;
        let t322 = t124 * rho[ip];
        let t323 = 1.0 / t322;
        let t324 = t323 * t127;
        let t329 = t121 * t132;
        let t330 = t124 * t304;
        let t332 = 1.0 / t33 / t330;
        let t334 = t332 * t139 * t35;
        let t339 = t132 * sigma[ip];
        let t340 = t131 * t339;
        let t341 = t124 * t312;
        let t344 = t15 / t13 / t341;
        let t346 = 1.0 / t138 / t103;
        let t347 = t344 * t346;
        let t350 = -0.53333333333333333333e0 * t95 * t307 * t104 + 0.21333333333333333334e0 * t311 * t316 - 0.42666666666666666667e0 * t110 * t316 + 0.17066666666666666667e0 * t321 * t324 - 0.256e0 * t123 * t324 + 0.512e-1 * t329 * t334 - 0.68266666666666666667e-1 * t133 * t334 + 0.27306666666666666668e-1 * t340 * t347;
        let t351 = t92 * t350;
        let t356 = t158 * t158;
        let t357 = 1.0 / t356;
        let t358 = t149 * t357;
        let t360 = 1.0 / t150 * t6;
        let t361 = t8 * t11;
        let t362 = t361 * t218;
        let t363 = t360 * t362;
        let t365 = t9 * t230;
        let t367 = f64::sqrt(t147);
        let t368 = t367 * t6;
        let t369 = t368 * t362;
        let t372 = t31 * t10 * t244;
        let t374 = -0.632975e0 * t363 - 0.29896666666666666667e0 * t365 - 0.1023875e0 * t369 - 0.82156666666666666667e-1 * t372;
        let t375 = 1.0 / t161;
        let t376 = t374 * t375;
        let t379 = t168 * t6;
        let t384 = t168 * t170;
        let t385 = t175 * t175;
        let t386 = 1.0 / t385;
        let t391 = -0.86308333333333333334e0 * t363 - 0.301925e0 * t365 - 0.5501625e-1 * t369 - 0.82785e-1 * t372;
        let t393 = 1.0 / t178;
        let t394 = t386 * t391 * t393;
        let t398 = 0.11073577833333333333e-2 * t9 * t230 * t162 + 1.0 * t358 * t376 - 0.18311555036753159941e-3 * t379 * t361 * t218 * t179 - 0.58482233974552040708e0 * t384 * t394 - 2.0 * t301;
        let t399 = t398 * t215;
        let t403 = t186 * t109;
        let t404 = t315 * t197;
        let t409 = t194 * t122;
        let t410 = t323 * t204;
        let t415 = t201 * t132;
        let t417 = t332 * t211 * t35;
        let t422 = t208 * t339;
        let t424 = 1.0 / t210 / t189;
        let t425 = t344 * t424;
        let t428 = -0.16e-1 * t187 * t307 * t190 + 0.192e-3 * t403 * t404 - 0.384e-3 * t195 * t404 + 0.4608e-5 * t409 * t410 - 0.6912e-5 * t202 * t410 + 0.41472e-7 * t415 * t417 - 0.55296e-7 * t209 * t417 + 0.663552e-9 * t422 * t425;
        let t429 = t184 * t428;
        let tvrho0 = t145 + t216 + rho[ip] * (2.0 * t302 + 2.0 * t351 + t399 + t429);
        vrho[ip] += tvrho0;
        let t432 = t94 * t35;
        let t438 = t108 * sigma[ip];
        let t443 = t121 * t109;
        let t448 = t131 * t122;
        let t451 = t124 * t112;
        let t454 = t15 / t13 / t451;
        let t455 = t454 * t346;
        let t458 = 0.2e0 * t432 * t98 * t104 - 0.8e-1 * t95 * t118 + 0.16e0 * t438 * t118 - 0.64e-1 * t110 * t128 + 0.96e-1 * t443 * t128 - 0.192e-1 * t123 * t140 + 0.256e-1 * t448 * t140 - 0.1024e-1 * t133 * t455;
        let t460 = 2.0 * t92 * t458;
        let t461 = t186 * t35;
        let t467 = t194 * sigma[ip];
        let t472 = t201 * t109;
        let t477 = t208 * t122;
        let t480 = t454 * t424;
        let t483 = 0.6e-2 * t461 * t98 * t190 - 0.72e-4 * t187 * t198 + 0.144e-3 * t467 * t198 - 0.1728e-5 * t195 * t205 + 0.2592e-5 * t472 * t205 - 0.15552e-7 * t202 * t212 + 0.20736e-7 * t477 * t212 - 0.248832e-9 * t209 * t480;
        let t484 = t184 * t483;
        let tvsigma0 = rho[ip] * (t460 + t484);
        vsigma[ip] += tvsigma0;
    }
}
