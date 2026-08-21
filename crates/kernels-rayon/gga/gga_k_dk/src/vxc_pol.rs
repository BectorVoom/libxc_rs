//! GGA_K_DK vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_dk.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_dk_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_aa_1: f64,
    param_aa_2: f64,
    param_aa_3: f64,
    param_aa_4: f64,
    param_aa_0: f64,
    param_bb_1: f64,
    param_bb_2: f64,
    param_bb_3: f64,
    param_bb_4: f64,
    param_bb_0: f64,
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
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
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
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = t6 * t28;
        let t30 = pow_1_3(t7);
        let t31 = t30 * t30;
        let t32 = param_aa_0;
        let t33 = param_aa_1;
        let t34 = t33 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = param_aa_2;
        let t42 = sigma0 * sigma0;
        let t43 = t41 * t42;
        let t44 = t35 * t35;
        let t45 = t44 * rho0;
        let t47 = 1.0 / t36 / t45;
        let t49 = param_aa_3;
        let t50 = t42 * sigma0;
        let t51 = t49 * t50;
        let t52 = t44 * t44;
        let t53 = 1.0 / t52;
        let t55 = param_aa_4;
        let t56 = t42 * t42;
        let t57 = t55 * t56;
        let t58 = t52 * t35;
        let t60 = 1.0 / t37 / t58;
        let t62 = t34 * t39 + t43 * t47 + t51 * t53 + t57 * t60 + t32;
        let t63 = t31 * t62;
        let t64 = param_bb_0;
        let t65 = param_bb_1;
        let t66 = t65 * sigma0;
        let t68 = param_bb_2;
        let t69 = t68 * t42;
        let t71 = param_bb_3;
        let t72 = t71 * t50;
        let t74 = param_bb_4;
        let t75 = t74 * t56;
        let t77 = t66 * t39 + t69 * t47 + t72 * t53 + t75 * t60 + t64;
        let t78 = 1.0 / t77;
        let t79 = t63 * t78;
        let t82 = piecewise3(t1, 0.0, 3.0 / 20.0 * t29 * t79);
        let t83 = rho1 <= dens_threshold;
        let t84 = -t17;
        let t86 = piecewise5(t15, t12, t11, t16, t84 * t8);
        let t87 = 1.0 + t86;
        let t88 = t87 <= zeta_threshold;
        let t89 = pow_1_3(t87);
        let t90 = t89 * t89;
        let t92 = piecewise3(t88, t24, t90 * t87);
        let t93 = t6 * t92;
        let t94 = t33 * sigma2;
        let t95 = rho1 * rho1;
        let t96 = pow_1_3(rho1);
        let t97 = t96 * t96;
        let t99 = 1.0 / t97 / t95;
        let t101 = sigma2 * sigma2;
        let t102 = t41 * t101;
        let t103 = t95 * t95;
        let t104 = t103 * rho1;
        let t106 = 1.0 / t96 / t104;
        let t108 = t101 * sigma2;
        let t109 = t49 * t108;
        let t110 = t103 * t103;
        let t111 = 1.0 / t110;
        let t113 = t101 * t101;
        let t114 = t55 * t113;
        let t115 = t110 * t95;
        let t117 = 1.0 / t97 / t115;
        let t119 = t102 * t106 + t109 * t111 + t114 * t117 + t94 * t99 + t32;
        let t120 = t31 * t119;
        let t121 = t65 * sigma2;
        let t123 = t68 * t101;
        let t125 = t71 * t108;
        let t127 = t74 * t113;
        let t129 = t123 * t106 + t125 * t111 + t127 * t117 + t121 * t99 + t64;
        let t130 = 1.0 / t129;
        let t131 = t120 * t130;
        let t134 = piecewise3(t83, 0.0, 3.0 / 20.0 * t93 * t131);
        let tzk0 = t82 + t134;
        zk[ip] += tzk0;
        let t135 = t7 * t7;
        let t136 = 1.0 / t135;
        let t137 = t17 * t136;
        let t139 = piecewise5(t11, 0.0, t15, 0.0, t8 - t137);
        let t142 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t139);
        let t143 = t6 * t142;
        let t146 = 1.0 / t30;
        let t147 = t146 * t62;
        let t148 = t147 * t78;
        let t150 = t29 * t148 / 10.0;
        let t151 = t35 * rho0;
        let t153 = 1.0 / t37 / t151;
        let t156 = t44 * t35;
        let t158 = 1.0 / t36 / t156;
        let t161 = t52 * rho0;
        let t162 = 1.0 / t161;
        let t165 = t52 * t151;
        let t167 = 1.0 / t37 / t165;
        let t170 = -8.0 / 3.0 * t34 * t153 - 16.0 / 3.0 * t43 * t158 - 8.0 * t51 * t162 - 32.0 / 3.0 * t57 * t167;
        let t171 = t31 * t170;
        let t172 = t171 * t78;
        let t175 = t77 * t77;
        let t176 = 1.0 / t175;
        let t185 = -8.0 / 3.0 * t66 * t153 - 16.0 / 3.0 * t69 * t158 - 8.0 * t72 * t162 - 32.0 / 3.0 * t75 * t167;
        let t186 = t176 * t185;
        let t187 = t63 * t186;
        let t191 = piecewise3(t1, 0.0, 3.0 / 20.0 * t143 * t79 + t150 + 3.0 / 20.0 * t29 * t172 - 3.0 / 20.0 * t29 * t187);
        let t192 = t84 * t136;
        let t194 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t192);
        let t197 = piecewise3(t88, 0.0, 5.0 / 3.0 * t90 * t194);
        let t198 = t6 * t197;
        let t201 = t146 * t119;
        let t202 = t201 * t130;
        let t204 = t93 * t202 / 10.0;
        let t206 = piecewise3(t83, 0.0, 3.0 / 20.0 * t198 * t131 + t204);
        let tvrho0 = t82 + t134 + t7 * (t191 + t206);
        vrho[ip * 2] += tvrho0;
        let t210 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t137);
        let t213 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t210);
        let t214 = t6 * t213;
        let t218 = piecewise3(t1, 0.0, 3.0 / 20.0 * t214 * t79 + t150);
        let t220 = piecewise5(t15, 0.0, t11, 0.0, t8 - t192);
        let t223 = piecewise3(t88, 0.0, 5.0 / 3.0 * t90 * t220);
        let t224 = t6 * t223;
        let t227 = t95 * rho1;
        let t229 = 1.0 / t97 / t227;
        let t232 = t103 * t95;
        let t234 = 1.0 / t96 / t232;
        let t237 = t110 * rho1;
        let t238 = 1.0 / t237;
        let t241 = t110 * t227;
        let t243 = 1.0 / t97 / t241;
        let t246 = -8.0 / 3.0 * t94 * t229 - 16.0 / 3.0 * t102 * t234 - 8.0 * t109 * t238 - 32.0 / 3.0 * t114 * t243;
        let t247 = t31 * t246;
        let t248 = t247 * t130;
        let t251 = t129 * t129;
        let t252 = 1.0 / t251;
        let t261 = -8.0 / 3.0 * t121 * t229 - 16.0 / 3.0 * t123 * t234 - 8.0 * t125 * t238 - 32.0 / 3.0 * t127 * t243;
        let t262 = t252 * t261;
        let t263 = t120 * t262;
        let t267 = piecewise3(t83, 0.0, 3.0 / 20.0 * t224 * t131 + t204 + 3.0 / 20.0 * t93 * t248 - 3.0 / 20.0 * t93 * t263);
        let tvrho1 = t82 + t134 + t7 * (t218 + t267);
        vrho[ip * 2 + 1] += tvrho1;
        let t271 = t41 * sigma0;
        let t274 = t49 * t42;
        let t277 = t55 * t50;
        let t280 = 2.0 * t271 * t47 + 3.0 * t274 * t53 + 4.0 * t277 * t60 + t33 * t39;
        let t281 = t31 * t280;
        let t282 = t281 * t78;
        let t285 = t68 * sigma0;
        let t288 = t71 * t42;
        let t291 = t74 * t50;
        let t294 = 2.0 * t285 * t47 + 3.0 * t288 * t53 + 4.0 * t291 * t60 + t65 * t39;
        let t295 = t176 * t294;
        let t296 = t63 * t295;
        let t300 = piecewise3(t1, 0.0, 3.0 / 20.0 * t29 * t282 - 3.0 / 20.0 * t29 * t296);
        let tvsigma0 = t7 * t300;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t302 = t41 * sigma2;
        let t305 = t49 * t101;
        let t308 = t55 * t108;
        let t311 = 2.0 * t302 * t106 + 3.0 * t305 * t111 + 4.0 * t308 * t117 + t33 * t99;
        let t312 = t31 * t311;
        let t313 = t312 * t130;
        let t316 = t68 * sigma2;
        let t319 = t71 * t101;
        let t322 = t74 * t108;
        let t325 = 2.0 * t316 * t106 + 3.0 * t319 * t111 + 4.0 * t322 * t117 + t65 * t99;
        let t326 = t252 * t325;
        let t327 = t120 * t326;
        let t331 = piecewise3(t83, 0.0, 3.0 / 20.0 * t93 * t313 - 3.0 / 20.0 * t93 * t327);
        let tvsigma2 = t7 * t331;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
