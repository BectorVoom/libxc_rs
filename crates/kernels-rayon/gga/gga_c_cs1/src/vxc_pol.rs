//! GGA_C_CS1 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_cs1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_cs1_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t11 = 1.0 + 0.349 * t9;
        let t12 = 1.0 / t11;
        let t13 = t7 * t12;
        let t15 = sigma0 + 2.0 * sigma1 + sigma2;
        let t16 = t15 * t15;
        let t17 = t4 * t4;
        let t18 = t17 * t3;
        let t20 = 1.0 / t8 / t18;
        let t22 = t8 * t8;
        let t24 = 1.0 / t22 / t4;
        let t27 = 1.0 + 0.006 * t15 * t24;
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t32 = -0.159068 + 2.86308e-07 * t16 * t20 * t29;
        let t34 = t13 * t32 / 4.0;
        let t35 = 1.0 / t3;
        let t36 = t1 * t35;
        let t37 = 1.0 + t36;
        let t38 = t37 <= zeta_threshold;
        let t39 = piecewise3(t38, zeta_threshold, t37);
        let t40 = pow_1_3(rho0);
        let t41 = t39 * t40;
        let t42 = t40 + 0.349;
        let t43 = 1.0 / t42;
        let t44 = sigma0 * sigma0;
        let t45 = rho0 * rho0;
        let t46 = t45 * t45;
        let t47 = t46 * rho0;
        let t49 = 1.0 / t40 / t47;
        let t51 = t40 * t40;
        let t53 = 1.0 / t51 / t45;
        let t56 = 1.0 + 0.006 * sigma0 * t53;
        let t57 = t56 * t56;
        let t58 = 1.0 / t57;
        let t61 = -0.018897 + 5.58864e-06 * t44 * t49 * t58;
        let t62 = t43 * t61;
        let t64 = t41 * t62 / 2.0;
        let t65 = 1.0 - t36;
        let t66 = t65 <= zeta_threshold;
        let t67 = piecewise3(t66, zeta_threshold, t65);
        let t68 = pow_1_3(rho1);
        let t69 = t67 * t68;
        let t70 = t68 + 0.349;
        let t71 = 1.0 / t70;
        let t72 = sigma2 * sigma2;
        let t73 = rho1 * rho1;
        let t74 = t73 * t73;
        let t75 = t74 * rho1;
        let t77 = 1.0 / t68 / t75;
        let t79 = t68 * t68;
        let t81 = 1.0 / t79 / t73;
        let t84 = 1.0 + 0.006 * sigma2 * t81;
        let t85 = t84 * t84;
        let t86 = 1.0 / t85;
        let t89 = -0.018897 + 5.58864e-06 * t72 * t77 * t86;
        let t90 = t71 * t89;
        let t92 = t69 * t90 / 2.0;
        let tzk0 = t34 + t64 + t92;
        zk[ip] += tzk0;
        let t93 = t1 * t5;
        let t94 = t4 * t3;
        let t95 = 1.0 / t94;
        let t96 = t2 * t95;
        let t98 = -2.0 * t93 + 2.0 * t96;
        let t99 = t98 * t12;
        let t100 = t99 * t32;
        let t101 = t100 / 4.0;
        let t102 = t11 * t11;
        let t103 = 1.0 / t102;
        let t104 = t7 * t103;
        let t106 = 1.0 / t8 / t3;
        let t107 = t32 * t106;
        let t108 = t104 * t107;
        let t109 = 0.029083333333333333 * t108;
        let t110 = t17 * t4;
        let t112 = 1.0 / t8 / t110;
        let t116 = t16 * t15;
        let t117 = t17 * t17;
        let t118 = t117 * t3;
        let t119 = 1.0 / t118;
        let t122 = 1.0 / t28 / t27;
        let t125 = -1.526976e-06 * t16 * t112 * t29 + 9.161856e-09 * t116 * t119 * t122;
        let t126 = t13 * t125;
        let t127 = t126 / 4.0;
        let t128 = t35 - t93;
        let t129 = piecewise3(t38, 0.0, t128);
        let t130 = t129 * t40;
        let t131 = t130 * t62;
        let t132 = t131 / 2.0;
        let t133 = 1.0 / t51;
        let t134 = t39 * t133;
        let t135 = t134 * t62;
        let t136 = t135 / 6.0;
        let t137 = 1.0 / t40;
        let t138 = t39 * t137;
        let t139 = t42 * t42;
        let t140 = 1.0 / t139;
        let t141 = t140 * t61;
        let t142 = t138 * t141;
        let t143 = t142 / 6.0;
        let t144 = t46 * t45;
        let t146 = 1.0 / t40 / t144;
        let t150 = t44 * sigma0;
        let t151 = t46 * t46;
        let t152 = t151 * rho0;
        let t153 = 1.0 / t152;
        let t156 = 1.0 / t57 / t56;
        let t159 = -2.980608e-05 * t44 * t146 * t58 + 1.7883648e-07 * t150 * t153 * t156;
        let t160 = t43 * t159;
        let t161 = t41 * t160;
        let t162 = t161 / 2.0;
        let t164 = piecewise3(t66, 0.0, -t128);
        let t165 = t164 * t68;
        let t166 = t165 * t90;
        let t167 = t166 / 2.0;
        let tvrho0 = t34 + t64 + t92 + t3 * (t101 + t109 + t127 + t132 + t136 - t143 + t162 + t167);
        vrho[ip * 2] += tvrho0;
        let t171 = 2.0 * t93 + 2.0 * t96;
        let t172 = t171 * t12;
        let t173 = t172 * t32;
        let t174 = t173 / 4.0;
        let t175 = -t35 - t93;
        let t176 = piecewise3(t38, 0.0, t175);
        let t177 = t176 * t40;
        let t178 = t177 * t62;
        let t179 = t178 / 2.0;
        let t181 = piecewise3(t66, 0.0, -t175);
        let t182 = t181 * t68;
        let t183 = t182 * t90;
        let t184 = t183 / 2.0;
        let t185 = 1.0 / t79;
        let t186 = t67 * t185;
        let t187 = t186 * t90;
        let t188 = t187 / 6.0;
        let t189 = 1.0 / t68;
        let t190 = t67 * t189;
        let t191 = t70 * t70;
        let t192 = 1.0 / t191;
        let t193 = t192 * t89;
        let t194 = t190 * t193;
        let t195 = t194 / 6.0;
        let t196 = t74 * t73;
        let t198 = 1.0 / t68 / t196;
        let t202 = t72 * sigma2;
        let t203 = t74 * t74;
        let t204 = t203 * rho1;
        let t205 = 1.0 / t204;
        let t208 = 1.0 / t85 / t84;
        let t211 = -2.980608e-05 * t72 * t198 * t86 + 1.7883648e-07 * t202 * t205 * t208;
        let t212 = t71 * t211;
        let t213 = t69 * t212;
        let t214 = t213 / 2.0;
        let tvrho1 = t34 + t64 + t92 + t3 * (t174 + t109 + t127 + t179 + t184 + t188 - t195 + t214);
        vrho[ip * 2 + 1] += tvrho1;
        let t218 = t15 * t20 * t29;
        let t220 = 1.0 / t117;
        let t222 = t16 * t220 * t122;
        let t224 = 5.72616e-07 * t218 - 3.435696e-09 * t222;
        let t226 = t13 * t224 / 4.0;
        let t230 = 1.0 / t151;
        let t234 = 1.117728e-05 * sigma0 * t49 * t58 - 6.706368e-08 * t44 * t230 * t156;
        let t235 = t43 * t234;
        let t237 = t41 * t235 / 2.0;
        let tvsigma0 = t3 * (t226 + t237);
        vsigma[ip * 3] += tvsigma0;
        let t239 = t3 * t7;
        let t242 = 1.145232e-06 * t218 - 6.871392e-09 * t222;
        let t243 = t12 * t242;
        let tvsigma1 = t239 * t243 / 4.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t248 = 1.0 / t203;
        let t252 = 1.117728e-05 * sigma2 * t77 * t86 - 6.706368e-08 * t72 * t248 * t208;
        let t253 = t71 * t252;
        let t255 = t69 * t253 / 2.0;
        let tvsigma2 = t3 * (t226 + t255);
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
