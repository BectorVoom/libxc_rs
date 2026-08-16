//! MGGA_X_JK vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_jk_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = t3 * t3;
        let t30 = param_beta * t29;
        let t32 = pow_1_3(1.0 / M_PI);
        let t33 = 1.0 / t32;
        let t34 = M_CBRT4;
        let t35 = t33 * t34;
        let t36 = t30 * t35;
        let t37 = rho0 * rho0;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t40 = t39 * t37;
        let t41 = 1.0 / t40;
        let t42 = sigma0 * t41;
        let t43 = param_gamma * param_beta;
        let t44 = f64::sqrt(sigma0);
        let t45 = t38 * rho0;
        let t46 = 1.0 / t45;
        let t47 = t44 * t46;
        let t48 = f64::ln(t47 + f64::sqrt(t47 * t47 + 1.0));
        let t51 = t43 * t47 * t48 + 1.0;
        let t52 = 1.0 / t51;
        let t53 = t39 * rho0;
        let t54 = 1.0 / t53;
        let t56 = -lapl0 * t54 + t42;
        let t57 = 1.0 / sigma0;
        let t58 = t56 * t57;
        let t61 = 2.0 * t40 * t58 + 1.0;
        let t62 = 1.0 / t61;
        let t63 = t52 * t62;
        let t67 = 1.0 + 2.0 / 9.0 * t36 * t42 * t63;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t67);
        let t72 = rho1 <= dens_threshold;
        let t73 = -t17;
        let t75 = piecewise5(t15, t12, t11, t16, t73 * t8);
        let t76 = 1.0 + t75;
        let t77 = t76 <= zeta_threshold;
        let t78 = pow_1_3(t76);
        let t80 = piecewise3(t77, t23, t78 * t76);
        let t81 = t80 * t27;
        let t82 = rho1 * rho1;
        let t83 = pow_1_3(rho1);
        let t84 = t83 * t83;
        let t85 = t84 * t82;
        let t86 = 1.0 / t85;
        let t87 = sigma2 * t86;
        let t88 = f64::sqrt(sigma2);
        let t89 = t83 * rho1;
        let t90 = 1.0 / t89;
        let t91 = t88 * t90;
        let t92 = f64::ln(t91 + f64::sqrt(t91 * t91 + 1.0));
        let t95 = t43 * t91 * t92 + 1.0;
        let t96 = 1.0 / t95;
        let t97 = t84 * rho1;
        let t98 = 1.0 / t97;
        let t100 = -lapl1 * t98 + t87;
        let t101 = 1.0 / sigma2;
        let t102 = t100 * t101;
        let t105 = 2.0 * t102 * t85 + 1.0;
        let t106 = 1.0 / t105;
        let t107 = t96 * t106;
        let t111 = 1.0 + 2.0 / 9.0 * t36 * t87 * t107;
        let t115 = piecewise3(t72, 0.0, -3.0 / 8.0 * t6 * t81 * t111);
        let tzk0 = t71 + t115;
        zk[ip] += tzk0;
        let t116 = t7 * t7;
        let t117 = 1.0 / t116;
        let t118 = t17 * t117;
        let t120 = piecewise5(t11, 0.0, t15, 0.0, t8 - t118);
        let t123 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t120);
        let t124 = t123 * t27;
        let t128 = t27 * t27;
        let t129 = 1.0 / t128;
        let t130 = t26 * t129;
        let t133 = t6 * t130 * t67 / 8.0;
        let t134 = t37 * rho0;
        let t136 = 1.0 / t39 / t134;
        let t137 = sigma0 * t136;
        let t141 = t51 * t51;
        let t142 = 1.0 / t141;
        let t143 = t142 * t62;
        let t145 = 1.0 / t38 / t37;
        let t149 = t42 + 1.0;
        let t150 = f64::sqrt(t149);
        let t151 = 1.0 / t150;
        let t155 = -4.0 / 3.0 * t43 * t44 * t145 * t48 - 4.0 / 3.0 * t43 * t137 * t151;
        let t156 = t143 * t155;
        let t160 = t61 * t61;
        let t161 = 1.0 / t160;
        let t162 = t52 * t161;
        let t166 = -8.0 / 3.0 * t137 + 5.0 / 3.0 * lapl0 * t41;
        let t167 = t166 * t57;
        let t172 = 2.0 * t167 * t40 + 16.0 / 3.0 * t58 * t53;
        let t173 = t162 * t172;
        let t177 = -16.0 / 27.0 * t36 * t137 * t63 - 2.0 / 9.0 * t36 * t42 * t156 - 2.0 / 9.0 * t36 * t42 * t173;
        let t182 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t124 * t67 - t133 - 3.0 / 8.0 * t6 * t28 * t177);
        let t183 = t73 * t117;
        let t185 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t183);
        let t188 = piecewise3(t77, 0.0, 4.0 / 3.0 * t78 * t185);
        let t189 = t188 * t27;
        let t193 = t80 * t129;
        let t196 = t6 * t193 * t111 / 8.0;
        let t198 = piecewise3(t72, 0.0, -3.0 / 8.0 * t6 * t189 * t111 - t196);
        let tvrho0 = t71 + t115 + t7 * (t182 + t198);
        vrho[ip * 2] += tvrho0;
        let t202 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t118);
        let t205 = piecewise3(t21, 0.0, 4.0 / 3.0 * t24 * t202);
        let t206 = t205 * t27;
        let t211 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t206 * t67 - t133);
        let t213 = piecewise5(t15, 0.0, t11, 0.0, t8 - t183);
        let t216 = piecewise3(t77, 0.0, 4.0 / 3.0 * t78 * t213);
        let t217 = t216 * t27;
        let t221 = t82 * rho1;
        let t223 = 1.0 / t84 / t221;
        let t224 = sigma2 * t223;
        let t228 = t95 * t95;
        let t229 = 1.0 / t228;
        let t230 = t229 * t106;
        let t232 = 1.0 / t83 / t82;
        let t236 = t87 + 1.0;
        let t237 = f64::sqrt(t236);
        let t238 = 1.0 / t237;
        let t242 = -4.0 / 3.0 * t43 * t88 * t232 * t92 - 4.0 / 3.0 * t43 * t224 * t238;
        let t243 = t230 * t242;
        let t247 = t105 * t105;
        let t248 = 1.0 / t247;
        let t249 = t96 * t248;
        let t253 = -8.0 / 3.0 * t224 + 5.0 / 3.0 * lapl1 * t86;
        let t254 = t253 * t101;
        let t259 = 2.0 * t254 * t85 + 16.0 / 3.0 * t102 * t97;
        let t260 = t249 * t259;
        let t264 = -16.0 / 27.0 * t36 * t224 * t107 - 2.0 / 9.0 * t36 * t87 * t243 - 2.0 / 9.0 * t36 * t87 * t260;
        let t269 = piecewise3(t72, 0.0, -3.0 / 8.0 * t6 * t217 * t111 - t196 - 3.0 / 8.0 * t6 * t81 * t264);
        let tvrho1 = t71 + t115 + t7 * (t211 + t269);
        vrho[ip * 2 + 1] += tvrho1;
        let t272 = t30 * t33;
        let t273 = t34 * t41;
        let t276 = 1.0 / t44;
        let t283 = t43 * t276 * t46 * t48 / 2.0 + t43 * t41 * t151 / 2.0;
        let t284 = t143 * t283;
        let t287 = sigma0 * sigma0;
        let t288 = 1.0 / t287;
        let t289 = t56 * t288;
        let t292 = -2.0 * t289 * t40 + 2.0 * t57;
        let t293 = t162 * t292;
        let t297 = 2.0 / 9.0 * t272 * t273 * t63 - 2.0 / 9.0 * t36 * t42 * t284 - 2.0 / 9.0 * t36 * t42 * t293;
        let t301 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t297);
        let tvsigma0 = t7 * t301;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t302 = t34 * t86;
        let t305 = 1.0 / t88;
        let t312 = t43 * t305 * t90 * t92 / 2.0 + t43 * t86 * t238 / 2.0;
        let t313 = t230 * t312;
        let t316 = sigma2 * sigma2;
        let t317 = 1.0 / t316;
        let t318 = t100 * t317;
        let t321 = -2.0 * t318 * t85 + 2.0 * t101;
        let t322 = t249 * t321;
        let t326 = 2.0 / 9.0 * t272 * t302 * t107 - 2.0 / 9.0 * t36 * t87 * t313 - 2.0 / 9.0 * t36 * t87 * t322;
        let t330 = piecewise3(t72, 0.0, -3.0 / 8.0 * t6 * t81 * t326);
        let tvsigma2 = t7 * t330;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t331 = t5 * t26;
        let t332 = t27 * param_beta;
        let t333 = t331 * t332;
        let t336 = t35 * t54 * t52 * t161;
        let t339 = piecewise3(t2, 0.0, -t333 * t336 / 2.0);
        let tvlapl0 = t7 * t339;
        vlapl[ip * 2] += tvlapl0;
        let t340 = t5 * t80;
        let t341 = t340 * t332;
        let t344 = t35 * t98 * t96 * t248;
        let t347 = piecewise3(t72, 0.0, -t341 * t344 / 2.0);
        let tvlapl1 = t7 * t347;
        vlapl[ip * 2 + 1] += tvlapl1;
        let tvtau0 = 0.0;
        vtau[ip * 2] += tvtau0;
        let tvtau1 = 0.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
