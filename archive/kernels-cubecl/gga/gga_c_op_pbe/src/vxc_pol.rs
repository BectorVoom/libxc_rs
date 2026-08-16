//! GGA_C_OP_PBE vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_pbe_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = f64::abs(t4);
        let t11 = 1.0 - t5 <= zeta_threshold || rho0 <= dens_threshold && rho1 <= dens_threshold;
        let t13 = 1.0 + t4 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = 1.0 - t4 <= zeta_threshold;
        let t17 = -t14;
        let t18 = piecewise5::<f64>(t13, t14, t16, t17, t4);
        let t19 = t18 * t18;
        let t20 = 1.0 - t19;
        let t21 = t20 * t2;
        let t24 = 2.0 * rho0 * t3 <= zeta_threshold;
        let t27 = 2.0 * rho1 * t3 <= zeta_threshold;
        let t28 = piecewise5::<f64>(t24, t14, t27, t17, t4);
        let t29 = 1.0 + t28;
        let t32 = t29 * t2 / 2.0 <= dens_threshold;
        let t33 = M_CBRT3;
        let t34 = t33 * t33;
        let t36 = pow_1_3::<f64>(1.0 / M_PI);
        let t38 = t34 / t36;
        let t39 = M_CBRT4;
        let t40 = t38 * t39;
        let t41 = M_CBRT2;
        let t42 = t29 <= zeta_threshold;
        let t43 = 1.0 - t28;
        let t44 = t43 <= zeta_threshold;
        let t45 = piecewise5::<f64>(t42, t14, t44, t17, t28);
        let t46 = 1.0 + t45;
        let t47 = t46 * t2;
        let t48 = pow_1_3::<f64>(t47);
        let t49 = 1.0 / t48;
        let t51 = M_CBRT6;
        let t52 = M_PI * M_PI;
        let t53 = pow_1_3::<f64>(t52);
        let t54 = t53 * t53;
        let t55 = 1.0 / t54;
        let t56 = t51 * t55;
        let t57 = rho0 * rho0;
        let t58 = pow_1_3::<f64>(rho0);
        let t59 = t58 * t58;
        let t61 = 1.0 / t59 / t57;
        let t65 = 0.804e0 + 0.91464571985215458336e-2 * t56 * sigma0 * t61;
        let t68 = 0.1804e1 - 0.646416e0 / t65;
        let t69 = 1.0 / t68;
        let t73 = piecewise3::<f64>(t32, 0.0, t40 * t41 * t49 * t69 / 9.0);
        let t77 = t43 * t2 / 2.0 <= dens_threshold;
        let t78 = piecewise5::<f64>(t44, t14, t42, t17, -t28);
        let t79 = 1.0 + t78;
        let t80 = t79 * t2;
        let t81 = pow_1_3::<f64>(t80);
        let t82 = 1.0 / t81;
        let t84 = rho1 * rho1;
        let t85 = pow_1_3::<f64>(rho1);
        let t86 = t85 * t85;
        let t88 = 1.0 / t86 / t84;
        let t92 = 0.804e0 + 0.91464571985215458336e-2 * t56 * sigma2 * t88;
        let t95 = 0.1804e1 - 0.646416e0 / t92;
        let t96 = 1.0 / t95;
        let t100 = piecewise3::<f64>(t77, 0.0, t40 * t41 * t82 * t96 / 9.0);
        let t101 = t73 + t100;
        let t102 = t101 == 0.0;
        let t103 = piecewise3::<f64>(t102, f64::EPSILON, t101);
        let t106 = 0.361925846e1 / t103 + 0.5764e0;
        let t107 = t103 * t103;
        let t108 = t107 * t107;
        let t109 = 1.0 / t108;
        let t111 = t107 * t103;
        let t112 = 1.0 / t111;
        let t114 = 1.0 / t107;
        let t116 = 0.320261508740743441e2 * t109 + 0.151911844324290596e2 * t112 + 0.1801312286343e1 * t114;
        let t117 = 1.0 / t116;
        let t118 = t106 * t117;
        let tzk0 = piecewise3::<f64>(t11, 0.0, -0.25e0 * t21 * t118);
        zk[ip] += tzk0;
        let t121 = t2 * t2;
        let t122 = 1.0 / t121;
        let t123 = t1 * t122;
        let t124 = t3 - t123;
        let t125 = piecewise5::<f64>(t13, 0.0, t16, 0.0, t124);
        let t126 = t18 * t125;
        let t127 = t2 * t106;
        let t128 = t127 * t117;
        let t131 = t20 * t106;
        let t133 = 0.25e0 * t131 * t117;
        let t135 = 1.0 / t48 / t47;
        let t136 = t41 * t135;
        let t137 = piecewise5::<f64>(t24, 0.0, t27, 0.0, t124);
        let t138 = piecewise5::<f64>(t42, 0.0, t44, 0.0, t137);
        let t140 = t138 * t2 + t45 + 1.0;
        let t145 = t39 * t41;
        let t147 = t38 * t145 * t49;
        let t148 = t68 * t68;
        let t149 = 1.0 / t148;
        let t150 = t65 * t65;
        let t151 = 1.0 / t150;
        let t152 = t149 * t151;
        let t153 = t152 * t51;
        let t154 = t55 * sigma0;
        let t155 = t57 * rho0;
        let t157 = 1.0 / t59 / t155;
        let t158 = t154 * t157;
        let t163 = piecewise3::<f64>(t32, 0.0, -t40 * t136 * t69 * t140 / 27.0 + 0.17518270448709640212e-2 * t147 * t153 * t158);
        let t165 = 1.0 / t81 / t80;
        let t166 = t41 * t165;
        let t167 = piecewise5::<f64>(t44, 0.0, t42, 0.0, -t137);
        let t169 = t167 * t2 + t78 + 1.0;
        let t174 = piecewise3::<f64>(t77, 0.0, -t40 * t166 * t96 * t169 / 27.0);
        let t176 = piecewise3::<f64>(t102, 0.0, t163 + t174);
        let t177 = t114 * t176;
        let t178 = t177 * t117;
        let t181 = t116 * t116;
        let t182 = 1.0 / t181;
        let t183 = t106 * t182;
        let t185 = 1.0 / t108 / t103;
        let t186 = t185 * t176;
        let t188 = t109 * t176;
        let t190 = t112 * t176;
        let t192 = -0.1281046034962973764e3 * t186 - 0.455735532972871788e2 * t188 - 0.3602624572686e1 * t190;
        let t193 = t183 * t192;
        let t197 = piecewise3::<f64>(t11, 0.0, 0.5e0 * t126 * t128 - t133 + 0.904814615e0 * t21 * t178 + 0.25e0 * t21 * t193);
        let tvrho0 = t2 * t197 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t199 = -t3 - t123;
        let t200 = piecewise5::<f64>(t13, 0.0, t16, 0.0, t199);
        let t201 = t18 * t200;
        let t204 = piecewise5::<f64>(t24, 0.0, t27, 0.0, t199);
        let t205 = piecewise5::<f64>(t42, 0.0, t44, 0.0, t204);
        let t207 = t205 * t2 + t45 + 1.0;
        let t212 = piecewise3::<f64>(t32, 0.0, -t40 * t136 * t69 * t207 / 27.0);
        let t213 = piecewise5::<f64>(t44, 0.0, t42, 0.0, -t204);
        let t215 = t213 * t2 + t78 + 1.0;
        let t221 = t38 * t145 * t82;
        let t222 = t95 * t95;
        let t223 = 1.0 / t222;
        let t224 = t92 * t92;
        let t225 = 1.0 / t224;
        let t226 = t223 * t225;
        let t227 = t226 * t51;
        let t228 = t55 * sigma2;
        let t229 = t84 * rho1;
        let t231 = 1.0 / t86 / t229;
        let t232 = t228 * t231;
        let t237 = piecewise3::<f64>(t77, 0.0, -t40 * t166 * t96 * t215 / 27.0 + 0.17518270448709640212e-2 * t221 * t227 * t232);
        let t239 = piecewise3::<f64>(t102, 0.0, t212 + t237);
        let t240 = t114 * t239;
        let t241 = t240 * t117;
        let t244 = t185 * t239;
        let t246 = t109 * t239;
        let t248 = t112 * t239;
        let t250 = -0.1281046034962973764e3 * t244 - 0.455735532972871788e2 * t246 - 0.3602624572686e1 * t248;
        let t251 = t183 * t250;
        let t255 = piecewise3::<f64>(t11, 0.0, 0.5e0 * t201 * t128 - t133 + 0.904814615e0 * t21 * t241 + 0.25e0 * t21 * t251);
        let tvrho1 = t2 * t255 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t261 = piecewise3::<f64>(t32, 0.0, -0.65693514182661150796e-3 * t147 * t152 * t56 * t61);
        let t262 = piecewise3::<f64>(t102, 0.0, t261);
        let t263 = t114 * t262;
        let t264 = t263 * t117;
        let t267 = t185 * t262;
        let t269 = t109 * t262;
        let t271 = t112 * t262;
        let t273 = -0.1281046034962973764e3 * t267 - 0.455735532972871788e2 * t269 - 0.3602624572686e1 * t271;
        let t274 = t183 * t273;
        let t278 = piecewise3::<f64>(t11, 0.0, 0.904814615e0 * t21 * t264 + 0.25e0 * t21 * t274);
        let tvsigma0 = t2 * t278;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t283 = piecewise3::<f64>(t77, 0.0, -0.65693514182661150796e-3 * t221 * t226 * t56 * t88);
        let t284 = piecewise3::<f64>(t102, 0.0, t283);
        let t285 = t114 * t284;
        let t286 = t285 * t117;
        let t289 = t185 * t284;
        let t291 = t109 * t284;
        let t293 = t112 * t284;
        let t295 = -0.1281046034962973764e3 * t289 - 0.455735532972871788e2 * t291 - 0.3602624572686e1 * t293;
        let t296 = t183 * t295;
        let t300 = piecewise3::<f64>(t11, 0.0, 0.904814615e0 * t21 * t286 + 0.25e0 * t21 * t296);
        let tvsigma2 = t2 * t300;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
