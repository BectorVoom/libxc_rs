//! GGA_X_EV93 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ev93.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ev93_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_b1: f64,
    param_b2: f64,
    param_b3: f64,
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
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = M_CBRT6;
        let t29 = param_a1 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = t34 * t39;
        let t43 = t28 * t28;
        let t44 = param_a2 * t43;
        let t46 = 1.0 / t31 / t30;
        let t47 = sigma0 * sigma0;
        let t48 = t46 * t47;
        let t49 = t35 * t35;
        let t50 = t49 * rho0;
        let t52 = 1.0 / t36 / t50;
        let t53 = t48 * t52;
        let t56 = t30 * t30;
        let t57 = 1.0 / t56;
        let t58 = param_a3 * t57;
        let t59 = t47 * sigma0;
        let t60 = t49 * t49;
        let t61 = 1.0 / t60;
        let t62 = t59 * t61;
        let t65 = 1.0 + t29 * t40 / 24.0 + t44 * t53 / 576.0 + t58 * t62 / 2304.0;
        let t66 = t27 * t65;
        let t67 = param_b1 * t28;
        let t70 = param_b2 * t43;
        let t73 = param_b3 * t57;
        let t76 = 1.0 + t67 * t40 / 24.0 + t70 * t53 / 576.0 + t73 * t62 / 2304.0;
        let t77 = 1.0 / t76;
        let t78 = t66 * t77;
        let t81 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t78);
        let t82 = rho1 <= dens_threshold;
        let t83 = -t16;
        let t85 = piecewise5(t14, t11, t10, t15, t83 * t7);
        let t86 = 1.0 + t85;
        let t87 = t86 <= zeta_threshold;
        let t88 = pow_1_3(t86);
        let t90 = piecewise3(t87, t22, t88 * t86);
        let t91 = t5 * t90;
        let t92 = t33 * sigma2;
        let t93 = rho1 * rho1;
        let t94 = pow_1_3(rho1);
        let t95 = t94 * t94;
        let t97 = 1.0 / t95 / t93;
        let t98 = t92 * t97;
        let t101 = sigma2 * sigma2;
        let t102 = t46 * t101;
        let t103 = t93 * t93;
        let t104 = t103 * rho1;
        let t106 = 1.0 / t94 / t104;
        let t107 = t102 * t106;
        let t110 = t101 * sigma2;
        let t111 = t103 * t103;
        let t112 = 1.0 / t111;
        let t113 = t110 * t112;
        let t116 = 1.0 + t29 * t98 / 24.0 + t44 * t107 / 576.0 + t58 * t113 / 2304.0;
        let t117 = t27 * t116;
        let t124 = 1.0 + t67 * t98 / 24.0 + t70 * t107 / 576.0 + t73 * t113 / 2304.0;
        let t125 = 1.0 / t124;
        let t126 = t117 * t125;
        let t129 = piecewise3(t82, 0.0, -3.0 / 8.0 * t91 * t126);
        let tzk0 = t81 + t129;
        zk[ip] += tzk0;
        let t130 = t6 * t6;
        let t131 = 1.0 / t130;
        let t132 = t16 * t131;
        let t134 = piecewise5(t10, 0.0, t14, 0.0, t7 - t132);
        let t137 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t134);
        let t138 = t5 * t137;
        let t141 = t27 * t27;
        let t142 = 1.0 / t141;
        let t143 = t142 * t65;
        let t144 = t143 * t77;
        let t146 = t26 * t144 / 8.0;
        let t147 = t35 * rho0;
        let t149 = 1.0 / t37 / t147;
        let t150 = t34 * t149;
        let t153 = t49 * t35;
        let t155 = 1.0 / t36 / t153;
        let t156 = t48 * t155;
        let t159 = t60 * rho0;
        let t160 = 1.0 / t159;
        let t161 = t59 * t160;
        let t164 = -t29 * t150 / 9.0 - t44 * t156 / 108.0 - t58 * t161 / 288.0;
        let t165 = t27 * t164;
        let t166 = t165 * t77;
        let t169 = t76 * t76;
        let t170 = 1.0 / t169;
        let t177 = -t67 * t150 / 9.0 - t70 * t156 / 108.0 - t73 * t161 / 288.0;
        let t178 = t170 * t177;
        let t179 = t66 * t178;
        let t183 = piecewise3(t1, 0.0, -3.0 / 8.0 * t138 * t78 - t146 - 3.0 / 8.0 * t26 * t166 + 3.0 / 8.0 * t26 * t179);
        let t184 = t83 * t131;
        let t186 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t184);
        let t189 = piecewise3(t87, 0.0, 4.0 / 3.0 * t88 * t186);
        let t190 = t5 * t189;
        let t193 = t142 * t116;
        let t194 = t193 * t125;
        let t196 = t91 * t194 / 8.0;
        let t198 = piecewise3(t82, 0.0, -3.0 / 8.0 * t190 * t126 - t196);
        let tvrho0 = t81 + t129 + t6 * (t183 + t198);
        vrho[ip * 2] += tvrho0;
        let t202 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t132);
        let t205 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t202);
        let t206 = t5 * t205;
        let t210 = piecewise3(t1, 0.0, -3.0 / 8.0 * t206 * t78 - t146);
        let t212 = piecewise5(t14, 0.0, t10, 0.0, t7 - t184);
        let t215 = piecewise3(t87, 0.0, 4.0 / 3.0 * t88 * t212);
        let t216 = t5 * t215;
        let t219 = t93 * rho1;
        let t221 = 1.0 / t95 / t219;
        let t222 = t92 * t221;
        let t225 = t103 * t93;
        let t227 = 1.0 / t94 / t225;
        let t228 = t102 * t227;
        let t231 = t111 * rho1;
        let t232 = 1.0 / t231;
        let t233 = t110 * t232;
        let t236 = -t29 * t222 / 9.0 - t44 * t228 / 108.0 - t58 * t233 / 288.0;
        let t237 = t27 * t236;
        let t238 = t237 * t125;
        let t241 = t124 * t124;
        let t242 = 1.0 / t241;
        let t249 = -t67 * t222 / 9.0 - t70 * t228 / 108.0 - t73 * t233 / 288.0;
        let t250 = t242 * t249;
        let t251 = t117 * t250;
        let t255 = piecewise3(t82, 0.0, -3.0 / 8.0 * t216 * t126 - t196 - 3.0 / 8.0 * t91 * t238 + 3.0 / 8.0 * t91 * t251);
        let tvrho1 = t81 + t129 + t6 * (t210 + t255);
        vrho[ip * 2 + 1] += tvrho1;
        let t258 = t33 * t39;
        let t261 = t46 * sigma0;
        let t262 = t261 * t52;
        let t265 = t47 * t61;
        let t268 = t29 * t258 / 24.0 + t44 * t262 / 288.0 + t58 * t265 / 768.0;
        let t269 = t27 * t268;
        let t270 = t269 * t77;
        let t278 = t67 * t258 / 24.0 + t70 * t262 / 288.0 + t73 * t265 / 768.0;
        let t279 = t170 * t278;
        let t280 = t66 * t279;
        let t284 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t270 + 3.0 / 8.0 * t26 * t280);
        let tvsigma0 = t6 * t284;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t285 = t33 * t97;
        let t288 = t46 * sigma2;
        let t289 = t288 * t106;
        let t292 = t101 * t112;
        let t295 = t29 * t285 / 24.0 + t44 * t289 / 288.0 + t58 * t292 / 768.0;
        let t296 = t27 * t295;
        let t297 = t296 * t125;
        let t305 = t67 * t285 / 24.0 + t70 * t289 / 288.0 + t73 * t292 / 768.0;
        let t306 = t242 * t305;
        let t307 = t117 * t306;
        let t311 = piecewise3(t82, 0.0, -3.0 / 8.0 * t91 * t297 + 3.0 / 8.0 * t91 * t307);
        let tvsigma2 = t6 * t311;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
