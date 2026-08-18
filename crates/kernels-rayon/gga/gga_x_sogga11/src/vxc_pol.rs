//! GGA_X_SOGGA11 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sogga11.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sogga11_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a_1: f64,
    param_mu: f64,
    param_kappa: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_a_0: f64,
    param_b_0: f64,
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
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = param_a_0;
        let t29 = param_a_1;
        let t30 = M_CBRT6;
        let t31 = param_mu * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t35 = 1.0 / t34;
        let t36 = t31 * t35;
        let t37 = 1.0 / param_kappa;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t46 = t36 * t38 * t43 / 24.0;
        let t47 = 1.0 + t46;
        let t49 = 1.0 - 1.0 / t47;
        let t51 = param_a_2;
        let t52 = t49 * t49;
        let t54 = param_a_3;
        let t55 = t52 * t49;
        let t57 = param_a_4;
        let t58 = t52 * t52;
        let t60 = param_a_5;
        let t63 = param_b_0;
        let t64 = param_b_1;
        let t65 = f64::exp(-t46);
        let t66 = 1.0 - t65;
        let t68 = param_b_2;
        let t69 = t66 * t66;
        let t71 = param_b_3;
        let t72 = t69 * t66;
        let t74 = param_b_4;
        let t75 = t69 * t69;
        let t77 = param_b_5;
        let t80 = t60 * t58 * t49 + t77 * t75 * t66 + t29 * t49 + t51 * t52 + t54 * t55 + t57 * t58 + t64 * t66 + t68 * t69 + t71 * t72 + t74 * t75 + t28 + t63;
        let t84 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t80);
        let t85 = rho1 <= dens_threshold;
        let t86 = -t16;
        let t88 = piecewise5(t14, t11, t10, t15, t86 * t7);
        let t89 = 1.0 + t88;
        let t90 = t89 <= zeta_threshold;
        let t91 = pow_1_3(t89);
        let t93 = piecewise3(t90, t22, t91 * t89);
        let t94 = t93 * t26;
        let t95 = t37 * sigma2;
        let t96 = rho1 * rho1;
        let t97 = pow_1_3(rho1);
        let t98 = t97 * t97;
        let t100 = 1.0 / t98 / t96;
        let t103 = t36 * t95 * t100 / 24.0;
        let t104 = 1.0 + t103;
        let t106 = 1.0 - 1.0 / t104;
        let t108 = t106 * t106;
        let t110 = t108 * t106;
        let t112 = t108 * t108;
        let t116 = f64::exp(-t103);
        let t117 = 1.0 - t116;
        let t119 = t117 * t117;
        let t121 = t119 * t117;
        let t123 = t119 * t119;
        let t127 = t60 * t112 * t106 + t77 * t123 * t117 + t29 * t106 + t51 * t108 + t54 * t110 + t57 * t112 + t64 * t117 + t68 * t119 + t71 * t121 + t74 * t123 + t28 + t63;
        let t131 = piecewise3(t85, 0.0, -3.0 / 8.0 * t5 * t94 * t127);
        let tzk0 = t84 + t131;
        zk[ip] += tzk0;
        let t132 = t6 * t6;
        let t133 = 1.0 / t132;
        let t134 = t16 * t133;
        let t136 = piecewise5(t10, 0.0, t14, 0.0, t7 - t134);
        let t139 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t136);
        let t140 = t139 * t26;
        let t144 = t26 * t26;
        let t145 = 1.0 / t144;
        let t146 = t25 * t145;
        let t149 = t5 * t146 * t80 / 8.0;
        let t150 = t47 * t47;
        let t151 = 1.0 / t150;
        let t152 = t29 * t151;
        let t153 = t152 * t31;
        let t154 = t35 * t37;
        let t155 = t39 * rho0;
        let t157 = 1.0 / t41 / t155;
        let t158 = sigma0 * t157;
        let t162 = t51 * t49;
        let t163 = t151 * param_mu;
        let t164 = t162 * t163;
        let t165 = t30 * t35;
        let t167 = t165 * t38 * t157;
        let t170 = t54 * t52;
        let t171 = t170 * t163;
        let t174 = t57 * t55;
        let t175 = t174 * t163;
        let t178 = t60 * t58;
        let t179 = t178 * t163;
        let t182 = t64 * param_mu;
        let t183 = t182 * t165;
        let t184 = t157 * t65;
        let t188 = t68 * t66;
        let t189 = t188 * t31;
        let t191 = t154 * t158 * t65;
        let t194 = t71 * t69;
        let t195 = t194 * t31;
        let t198 = t74 * t72;
        let t199 = t198 * t31;
        let t202 = t77 * t75;
        let t203 = t202 * t31;
        let t206 = -t153 * t154 * t158 / 9.0 - 2.0 / 9.0 * t164 * t167 - t171 * t167 / 3.0 - 4.0 / 9.0 * t175 * t167 - 5.0 / 9.0 * t179 * t167 - t183 * t38 * t184 / 9.0 - 2.0 / 9.0 * t189 * t191 - t195 * t191 / 3.0 - 4.0 / 9.0 * t199 * t191 - 5.0 / 9.0 * t203 * t191;
        let t211 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t140 * t80 - t149 - 3.0 / 8.0 * t5 * t27 * t206);
        let t212 = t86 * t133;
        let t214 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t212);
        let t217 = piecewise3(t90, 0.0, 4.0 / 3.0 * t91 * t214);
        let t218 = t217 * t26;
        let t222 = t93 * t145;
        let t225 = t5 * t222 * t127 / 8.0;
        let t227 = piecewise3(t85, 0.0, -3.0 / 8.0 * t5 * t218 * t127 - t225);
        let tvrho0 = t84 + t131 + t6 * (t211 + t227);
        vrho[ip * 2] += tvrho0;
        let t231 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t134);
        let t234 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t231);
        let t235 = t234 * t26;
        let t240 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t235 * t80 - t149);
        let t242 = piecewise5(t14, 0.0, t10, 0.0, t7 - t212);
        let t245 = piecewise3(t90, 0.0, 4.0 / 3.0 * t91 * t242);
        let t246 = t245 * t26;
        let t250 = t104 * t104;
        let t251 = 1.0 / t250;
        let t252 = t29 * t251;
        let t253 = t252 * t31;
        let t254 = t96 * rho1;
        let t256 = 1.0 / t98 / t254;
        let t257 = sigma2 * t256;
        let t261 = t51 * t106;
        let t262 = t251 * param_mu;
        let t263 = t261 * t262;
        let t265 = t165 * t95 * t256;
        let t268 = t54 * t108;
        let t269 = t268 * t262;
        let t272 = t57 * t110;
        let t273 = t272 * t262;
        let t276 = t60 * t112;
        let t277 = t276 * t262;
        let t280 = t256 * t116;
        let t284 = t68 * t117;
        let t285 = t284 * t31;
        let t287 = t154 * t257 * t116;
        let t290 = t71 * t119;
        let t291 = t290 * t31;
        let t294 = t74 * t121;
        let t295 = t294 * t31;
        let t298 = t77 * t123;
        let t299 = t298 * t31;
        let t302 = -t253 * t154 * t257 / 9.0 - 2.0 / 9.0 * t263 * t265 - t269 * t265 / 3.0 - 4.0 / 9.0 * t273 * t265 - 5.0 / 9.0 * t277 * t265 - t183 * t95 * t280 / 9.0 - 2.0 / 9.0 * t285 * t287 - t291 * t287 / 3.0 - 4.0 / 9.0 * t295 * t287 - 5.0 / 9.0 * t299 * t287;
        let t307 = piecewise3(t85, 0.0, -3.0 / 8.0 * t5 * t246 * t127 - t225 - 3.0 / 8.0 * t5 * t94 * t302);
        let tvrho1 = t84 + t131 + t6 * (t240 + t307);
        vrho[ip * 2 + 1] += tvrho1;
        let t310 = t152 * param_mu;
        let t312 = t165 * t37 * t43;
        let t323 = t182 * t30;
        let t325 = t154 * t43 * t65;
        let t336 = t310 * t312 / 24.0 + t164 * t312 / 12.0 + t171 * t312 / 8.0 + t175 * t312 / 6.0 + 5.0 / 24.0 * t179 * t312 + t323 * t325 / 24.0 + t189 * t325 / 12.0 + t195 * t325 / 8.0 + t199 * t325 / 6.0 + 5.0 / 24.0 * t203 * t325;
        let t340 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t336);
        let tvsigma0 = t6 * t340;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t341 = t252 * param_mu;
        let t343 = t165 * t37 * t100;
        let t355 = t154 * t100 * t116;
        let t366 = t341 * t343 / 24.0 + t263 * t343 / 12.0 + t269 * t343 / 8.0 + t273 * t343 / 6.0 + 5.0 / 24.0 * t277 * t343 + t323 * t355 / 24.0 + t285 * t355 / 12.0 + t291 * t355 / 8.0 + t295 * t355 / 6.0 + 5.0 / 24.0 * t299 * t355;
        let t370 = piecewise3(t85, 0.0, -3.0 / 8.0 * t5 * t94 * t366);
        let tvsigma2 = t6 * t370;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
