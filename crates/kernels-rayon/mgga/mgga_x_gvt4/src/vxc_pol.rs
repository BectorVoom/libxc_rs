//! MGGA_X_GVT4 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gvt4_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let t3 = M_CBRTPI;
        let t4 = 1.0 / t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = pow_1_3(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = pow_1_3(t18);
        let t24 = piecewise3(t19, t21, t22 * t18);
        let t25 = t4 * t24;
        let t26 = pow_1_3(t5);
        let t27 = t25 * t26;
        let t28 = rho0 * rho0;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / t28;
        let t33 = sigma0 * t32;
        let t36 = 1.0 / t30 / rho0;
        let t37 = tau0 * t36;
        let t39 = M_CBRT6;
        let t40 = t39 * t39;
        let t41 = M_PI * M_PI;
        let t42 = pow_1_3(t41);
        let t43 = t42 * t42;
        let t44 = t40 * t43;
        let t45 = 0.001120356 * t44;
        let t46 = 1.0 + 0.00186726 * t33 + 0.00373452 * t37 - t45;
        let t51 = 0.0037501956 * t44;
        let t52 = -0.003556788 * t33 + 0.012500652 * t37 - t51;
        let t53 = t46 * t46;
        let t54 = 1.0 / t53;
        let t56 = sigma0 * sigma0;
        let t57 = t28 * t28;
        let t58 = t57 * rho0;
        let t60 = 1.0 / t29 / t58;
        let t64 = 3.0 / 5.0 * t44;
        let t65 = 2.0 * t37 - t64;
        let t68 = t65 * t65;
        let t70 = -2.354518e-05 * t56 * t60 - 0.0001282732 * t33 * t65 + 0.0003574822 * t68;
        let t71 = t53 * t46;
        let t72 = 1.0 / t71;
        let t76 = pow_1_3(1.0 / M_PI);
        let t77 = 1.0 / t76;
        let t79 = M_CBRT4;
        let t80 = (-0.9800683 / t46 + t52 * t54 + t70 * t72) * t77 * t79;
        let t83 = piecewise3(t2, 0.0, t27 * t80 / 4.0);
        let t84 = rho1 <= dens_threshold;
        let t85 = -t15;
        let t87 = piecewise5(t13, t10, t9, t14, t85 * t6);
        let t88 = 1.0 + t87;
        let t89 = t88 <= zeta_threshold;
        let t90 = pow_1_3(t88);
        let t92 = piecewise3(t89, t21, t90 * t88);
        let t93 = t4 * t92;
        let t94 = t93 * t26;
        let t95 = rho1 * rho1;
        let t96 = pow_1_3(rho1);
        let t97 = t96 * t96;
        let t99 = 1.0 / t97 / t95;
        let t100 = sigma2 * t99;
        let t103 = 1.0 / t97 / rho1;
        let t104 = tau1 * t103;
        let t106 = 1.0 + 0.00186726 * t100 + 0.00373452 * t104 - t45;
        let t111 = -0.003556788 * t100 + 0.012500652 * t104 - t51;
        let t112 = t106 * t106;
        let t113 = 1.0 / t112;
        let t115 = sigma2 * sigma2;
        let t116 = t95 * t95;
        let t117 = t116 * rho1;
        let t119 = 1.0 / t96 / t117;
        let t123 = 2.0 * t104 - t64;
        let t126 = t123 * t123;
        let t128 = -2.354518e-05 * t115 * t119 - 0.0001282732 * t100 * t123 + 0.0003574822 * t126;
        let t129 = t112 * t106;
        let t130 = 1.0 / t129;
        let t134 = (-0.9800683 / t106 + t111 * t113 + t128 * t130) * t77 * t79;
        let t137 = piecewise3(t84, 0.0, t94 * t134 / 4.0);
        let tzk0 = t83 + t137;
        zk[ip] += tzk0;
        let t138 = t5 * t5;
        let t139 = 1.0 / t138;
        let t140 = t15 * t139;
        let t142 = piecewise5(t9, 0.0, t13, 0.0, t6 - t140);
        let t145 = piecewise3(t19, 0.0, 4.0 / 3.0 * t22 * t142);
        let t146 = t4 * t145;
        let t147 = t146 * t26;
        let t150 = t26 * t26;
        let t151 = 1.0 / t150;
        let t152 = t25 * t151;
        let t154 = t152 * t80 / 12.0;
        let t155 = t28 * rho0;
        let t157 = 1.0 / t30 / t155;
        let t158 = sigma0 * t157;
        let t160 = tau0 * t32;
        let t162 = -0.00497936 * t158 - 0.0062242 * t160;
        let t167 = 0.009484768 * t158 - 0.02083442 * t160;
        let t169 = t52 * t72;
        let t172 = t57 * t28;
        let t174 = 1.0 / t29 / t172;
        let t179 = sigma0 * t60;
        let t182 = t65 * tau0;
        let t185 = 0.00012557429333333333 * t56 * t174 + 0.00034206186666666666 * t158 * t65 + 0.0004275773333333333 * t179 * tau0 - 0.0023832146666666666 * t182 * t32;
        let t187 = t53 * t53;
        let t188 = 1.0 / t187;
        let t189 = t70 * t188;
        let t194 = (0.9800683 * t54 * t162 + t167 * t54 - 2.0 * t169 * t162 + t185 * t72 - 3.0 * t189 * t162) * t77 * t79;
        let t198 = piecewise3(t2, 0.0, t147 * t80 / 4.0 + t154 + t27 * t194 / 4.0);
        let t199 = t85 * t139;
        let t201 = piecewise5(t13, 0.0, t9, 0.0, -t6 - t199);
        let t204 = piecewise3(t89, 0.0, 4.0 / 3.0 * t90 * t201);
        let t205 = t4 * t204;
        let t206 = t205 * t26;
        let t209 = t93 * t151;
        let t211 = t209 * t134 / 12.0;
        let t213 = piecewise3(t84, 0.0, t206 * t134 / 4.0 + t211);
        let tvrho0 = t83 + t137 + t5 * (t198 + t213);
        vrho[ip * 2] += tvrho0;
        let t217 = piecewise5(t9, 0.0, t13, 0.0, -t6 - t140);
        let t220 = piecewise3(t19, 0.0, 4.0 / 3.0 * t22 * t217);
        let t221 = t4 * t220;
        let t222 = t221 * t26;
        let t226 = piecewise3(t2, 0.0, t222 * t80 / 4.0 + t154);
        let t228 = piecewise5(t13, 0.0, t9, 0.0, t6 - t199);
        let t231 = piecewise3(t89, 0.0, 4.0 / 3.0 * t90 * t228);
        let t232 = t4 * t231;
        let t233 = t232 * t26;
        let t236 = t95 * rho1;
        let t238 = 1.0 / t97 / t236;
        let t239 = sigma2 * t238;
        let t241 = tau1 * t99;
        let t243 = -0.00497936 * t239 - 0.0062242 * t241;
        let t248 = 0.009484768 * t239 - 0.02083442 * t241;
        let t250 = t111 * t130;
        let t253 = t116 * t95;
        let t255 = 1.0 / t96 / t253;
        let t260 = sigma2 * t119;
        let t263 = t123 * tau1;
        let t266 = 0.00012557429333333333 * t115 * t255 + 0.00034206186666666666 * t239 * t123 + 0.0004275773333333333 * t260 * tau1 - 0.0023832146666666666 * t263 * t99;
        let t268 = t112 * t112;
        let t269 = 1.0 / t268;
        let t270 = t128 * t269;
        let t275 = (0.9800683 * t113 * t243 + t248 * t113 - 2.0 * t250 * t243 + t266 * t130 - 3.0 * t270 * t243) * t77 * t79;
        let t279 = piecewise3(t84, 0.0, t233 * t134 / 4.0 + t211 + t94 * t275 / 4.0);
        let tvrho1 = t83 + t137 + t5 * (t226 + t279);
        vrho[ip * 2 + 1] += tvrho1;
        let t282 = t54 * t32;
        let t284 = t169 * t32;
        let t287 = t32 * t65;
        let t289 = -4.709036e-05 * t179 - 0.0001282732 * t287;
        let t291 = t189 * t32;
        let t295 = (-0.001726745666142 * t282 - 0.00373452 * t284 + t289 * t72 - 0.00560178 * t291) * t77 * t79;
        let t298 = piecewise3(t2, 0.0, t27 * t295 / 4.0);
        let tvsigma0 = t5 * t298;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t299 = t113 * t99;
        let t301 = t250 * t99;
        let t304 = t99 * t123;
        let t306 = -4.709036e-05 * t260 - 0.0001282732 * t304;
        let t308 = t270 * t99;
        let t312 = (-0.001726745666142 * t299 - 0.00373452 * t301 + t306 * t130 - 0.00560178 * t308) * t77 * t79;
        let t315 = piecewise3(t84, 0.0, t94 * t312 / 4.0);
        let tvsigma2 = t5 * t315;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t321 = 1.0 / t29 / t57;
        let t326 = -0.0002565464 * sigma0 * t321 + 0.0014299288 * t65 * t36;
        let t332 = (0.016160736667716 * t54 * t36 - 0.00746904 * t169 * t36 + t326 * t72 - 0.01120356 * t189 * t36) * t77 * t79;
        let t335 = piecewise3(t2, 0.0, t27 * t332 / 4.0);
        let tvtau0 = t5 * t335;
        vtau[ip * 2] += tvtau0;
        let t341 = 1.0 / t96 / t116;
        let t346 = -0.0002565464 * sigma2 * t341 + 0.0014299288 * t123 * t103;
        let t352 = (0.016160736667716 * t113 * t103 - 0.00746904 * t250 * t103 + t346 * t130 - 0.01120356 * t270 * t103) * t77 * t79;
        let t355 = piecewise3(t84, 0.0, t94 * t352 / 4.0);
        let tvtau1 = t5 * t355;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
