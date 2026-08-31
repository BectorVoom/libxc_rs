//! GGA_C_LYP vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lyp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_lyp_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = pow_1_3(t3);
        let t9 = 1.0 / t8;
        let t11 = param_d * t9 + 1.0;
        let t12 = 1.0 / t11;
        let t15 = rmath::exp(-param_c * t9);
        let t16 = param_b * t15;
        let t18 = sigma0 + 2.0 * sigma1 + sigma2;
        let t19 = t8 * t8;
        let t21 = 1.0 / t19 / t4;
        let t22 = t18 * t21;
        let t24 = param_d * t12 + param_c;
        let t25 = t24 * t9;
        let t27 = 47.0 - 7.0 * t25;
        let t30 = t7 * t27 / 72.0 - 2.0 / 3.0;
        let t32 = M_CBRT3;
        let t33 = t32 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = t33 * t36;
        let t38 = 1.0 / t3;
        let t39 = t1 * t38;
        let t40 = 1.0 + t39;
        let t41 = t40 <= zeta_threshold;
        let t42 = zeta_threshold * zeta_threshold;
        let t43 = pow_1_3(zeta_threshold);
        let t44 = t43 * t43;
        let t45 = t44 * t42;
        let t46 = t40 * t40;
        let t47 = pow_1_3(t40);
        let t48 = t47 * t47;
        let t49 = t48 * t46;
        let t50 = piecewise3(t41, t45, t49);
        let t51 = 1.0 - t39;
        let t52 = t51 <= zeta_threshold;
        let t53 = t51 * t51;
        let t54 = pow_1_3(t51);
        let t55 = t54 * t54;
        let t56 = t55 * t53;
        let t57 = piecewise3(t52, t45, t56);
        let t58 = t50 + t57;
        let t62 = M_CBRT2;
        let t63 = t62 * t7;
        let t65 = 5.0 / 2.0 - t25 / 18.0;
        let t66 = rho0 * rho0;
        let t67 = pow_1_3(rho0);
        let t68 = t67 * t67;
        let t70 = 1.0 / t68 / t66;
        let t71 = sigma0 * t70;
        let t72 = t71 * t50;
        let t73 = rho1 * rho1;
        let t74 = pow_1_3(rho1);
        let t75 = t74 * t74;
        let t77 = 1.0 / t75 / t73;
        let t78 = sigma2 * t77;
        let t79 = t78 * t57;
        let t80 = t72 + t79;
        let t81 = t65 * t80;
        let t84 = t25 - 11.0;
        let t86 = t44 * t42 * zeta_threshold;
        let t89 = piecewise3(t41, t86, t48 * t46 * t40);
        let t93 = piecewise3(t52, t86, t55 * t53 * t51);
        let t95 = t71 * t89 + t78 * t93;
        let t96 = t84 * t95;
        let t101 = piecewise3(t41, t42, t46);
        let t102 = t101 * sigma2;
        let t103 = t77 * t57;
        let t106 = piecewise3(t52, t42, t53);
        let t107 = t106 * sigma0;
        let t108 = t70 * t50;
        let t114 = -t22 * t30 - 3.0 / 20.0 * t37 * t7 * t58 + t63 * t81 / 32.0 + t63 * t96 / 576.0 - t62 * (2.0 / 3.0 * t72 + 2.0 / 3.0 * t79 - t102 * t103 / 4.0 - t107 * t108 / 4.0) / 8.0;
        let tzk0 = param_a * (t16 * t12 * t114 - t7 * t12);
        zk[ip] += tzk0;
        let t118 = t3 * param_a;
        let t119 = t1 * t5;
        let t120 = t4 * t3;
        let t121 = 1.0 / t120;
        let t122 = t2 * t121;
        let t124 = -2.0 * t119 + 2.0 * t122;
        let t126 = t11 * t11;
        let t127 = 1.0 / t126;
        let t128 = t7 * t127;
        let t130 = 1.0 / t8 / t3;
        let t131 = param_d * t130;
        let t133 = t128 * t131 / 3.0;
        let t134 = param_b * param_c;
        let t135 = t134 * t130;
        let t136 = t15 * t12;
        let t137 = t136 * t114;
        let t139 = t135 * t137 / 3.0;
        let t140 = t16 * t127;
        let t141 = t114 * param_d;
        let t144 = t140 * t141 * t130 / 3.0;
        let t146 = 1.0 / t19 / t120;
        let t147 = t18 * t146;
        let t149 = 8.0 / 3.0 * t147 * t30;
        let t151 = param_d * param_d;
        let t152 = t151 * t127;
        let t154 = 1.0 / t19 / t3;
        let t157 = t24 * t130 - t152 * t154;
        let t158 = 7.0 / 3.0 * t157;
        let t159 = t7 * t158;
        let t161 = t124 * t27 / 72.0 + t159 / 72.0;
        let t166 = t48 * t40;
        let t167 = t38 - t119;
        let t168 = t166 * t167;
        let t170 = piecewise3(t41, 0.0, 8.0 / 3.0 * t168);
        let t171 = t55 * t51;
        let t172 = -t167;
        let t173 = t171 * t172;
        let t175 = piecewise3(t52, 0.0, 8.0 / 3.0 * t173);
        let t176 = t170 + t175;
        let t180 = t62 * t124;
        let t183 = t157 / 54.0;
        let t184 = t183 * t80;
        let t186 = t63 * t184 / 32.0;
        let t189 = 1.0 / t68 / t66 / rho0;
        let t190 = sigma0 * t189;
        let t191 = t190 * t50;
        let t193 = t71 * t170;
        let t194 = t78 * t175;
        let t195 = -8.0 / 3.0 * t191 + t193 + t194;
        let t196 = t65 * t195;
        let t202 = -t157 / 3.0;
        let t203 = t202 * t95;
        let t205 = t63 * t203 / 576.0;
        let t210 = piecewise3(t41, 0.0, 11.0 / 3.0 * t49 * t167);
        let t214 = piecewise3(t52, 0.0, 11.0 / 3.0 * t56 * t172);
        let t216 = -8.0 / 3.0 * t190 * t89 + t71 * t210 + t78 * t214;
        let t217 = t84 * t216;
        let t225 = piecewise3(t41, 0.0, 2.0 * t40 * t167);
        let t226 = t225 * sigma2;
        let t229 = t77 * t175;
        let t234 = piecewise3(t52, 0.0, 2.0 * t51 * t172);
        let t235 = t234 * sigma0;
        let t238 = t189 * t50;
        let t241 = t70 * t170;
        let t247 = t149 - t22 * t161 - 3.0 / 20.0 * t37 * t124 * t58 - 3.0 / 20.0 * t37 * t7 * t176 + t180 * t81 / 32.0 + t186 + t63 * t196 / 32.0 + t180 * t96 / 576.0 + t205 + t63 * t217 / 576.0 - t62 * (-16.0 / 9.0 * t191 + 2.0 / 3.0 * t193 + 2.0 / 3.0 * t194 - t226 * t103 / 4.0 - t102 * t229 / 4.0 - t235 * t108 / 4.0 + 2.0 / 3.0 * t107 * t238 - t107 * t241 / 4.0) / 8.0;
        let t250 = t16 * t12 * t247 - t124 * t12 - t133 + t139 + t144;
        let tvrho0 = t118 * t250 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t253 = 2.0 * t119 + 2.0 * t122;
        let t257 = t253 * t27 / 72.0 + t159 / 72.0;
        let t262 = -t38 - t119;
        let t263 = t166 * t262;
        let t265 = piecewise3(t41, 0.0, 8.0 / 3.0 * t263);
        let t266 = -t262;
        let t267 = t171 * t266;
        let t269 = piecewise3(t52, 0.0, 8.0 / 3.0 * t267);
        let t270 = t265 + t269;
        let t274 = t62 * t253;
        let t277 = t71 * t265;
        let t280 = 1.0 / t75 / t73 / rho1;
        let t281 = sigma2 * t280;
        let t282 = t281 * t57;
        let t284 = t78 * t269;
        let t285 = t277 - 8.0 / 3.0 * t282 + t284;
        let t286 = t65 * t285;
        let t293 = piecewise3(t41, 0.0, 11.0 / 3.0 * t49 * t262);
        let t299 = piecewise3(t52, 0.0, 11.0 / 3.0 * t56 * t266);
        let t301 = t71 * t293 - 8.0 / 3.0 * t281 * t93 + t78 * t299;
        let t302 = t84 * t301;
        let t310 = piecewise3(t41, 0.0, 2.0 * t40 * t262);
        let t311 = t310 * sigma2;
        let t314 = t280 * t57;
        let t317 = t77 * t269;
        let t322 = piecewise3(t52, 0.0, 2.0 * t51 * t266);
        let t323 = t322 * sigma0;
        let t326 = t70 * t265;
        let t332 = t149 - t22 * t257 - 3.0 / 20.0 * t37 * t253 * t58 - 3.0 / 20.0 * t37 * t7 * t270 + t274 * t81 / 32.0 + t186 + t63 * t286 / 32.0 + t274 * t96 / 576.0 + t205 + t63 * t302 / 576.0 - t62 * (2.0 / 3.0 * t277 - 16.0 / 9.0 * t282 + 2.0 / 3.0 * t284 - t311 * t103 / 4.0 + 2.0 / 3.0 * t102 * t314 - t102 * t317 / 4.0 - t323 * t108 / 4.0 - t107 * t326 / 4.0) / 8.0;
        let t335 = t16 * t12 * t332 - t253 * t12 - t133 + t139 + t144;
        let tvrho1 = t118 * t335 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t337 = t118 * param_b;
        let t338 = t21 * t30;
        let t339 = t65 * t70;
        let t340 = t339 * t50;
        let t343 = t84 * t70;
        let t344 = t343 * t89;
        let t348 = t106 * t70;
        let t354 = -t338 + t63 * t340 / 32.0 + t63 * t344 / 576.0 - t62 * (2.0 / 3.0 * t108 - t348 * t50 / 4.0) / 8.0;
        let t355 = t136 * t354;
        let tvsigma0 = t337 * t355;
        vsigma[ip * 3] += tvsigma0;
        let t356 = t154 * param_a;
        let t357 = t356 * param_b;
        let t358 = t136 * t30;
        let tvsigma1 = -2.0 * t357 * t358;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t361 = t65 * t77;
        let t362 = t361 * t57;
        let t365 = t84 * t77;
        let t366 = t365 * t93;
        let t370 = t101 * t77;
        let t376 = -t338 + t63 * t362 / 32.0 + t63 * t366 / 576.0 - t62 * (2.0 / 3.0 * t103 - t370 * t57 / 4.0) / 8.0;
        let t377 = t136 * t376;
        let tvsigma2 = t337 * t377;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
