//! GGA_C_CHACHIYO vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_chachiyo.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_chachiyo_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
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
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3::<f64>(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = rho0 + rho1;
        let t10 = pow_1_3::<f64>(t9);
        let t11 = t8 * t10;
        let t14 = param_cp * t1;
        let t15 = t5 * t5;
        let t17 = t7 * t7;
        let t18 = 1.0 / t15 * t17;
        let t19 = t10 * t10;
        let t20 = t18 * t19;
        let t23 = 1.0 + t3 * t11 / 3.0 + t14 * t20 / 3.0;
        let t24 = f64::ln(t23);
        let t25 = param_ap * t24;
        let t26 = param_bf * t2;
        let t29 = param_cf * t1;
        let t32 = 1.0 + t26 * t11 / 3.0 + t29 * t20 / 3.0;
        let t33 = f64::ln(t32);
        let t35 = param_af * t33 - t25;
        let t36 = rho0 - rho1;
        let t37 = 1.0 / t9;
        let t38 = t36 * t37;
        let t39 = 1.0 + t38;
        let t40 = t39 <= zeta_threshold;
        let t41 = pow_1_3::<f64>(zeta_threshold);
        let t42 = t41 * t41;
        let t43 = pow_1_3::<f64>(t39);
        let t44 = t43 * t43;
        let t45 = piecewise3::<f64>(t40, t42, t44);
        let t46 = 1.0 - t38;
        let t47 = t46 <= zeta_threshold;
        let t48 = pow_1_3::<f64>(t46);
        let t49 = t48 * t48;
        let t50 = piecewise3::<f64>(t47, t42, t49);
        let t52 = t45 / 2.0 + t50 / 2.0;
        let t53 = t52 * t52;
        let t56 = -2.0 * t53 * t52 + 2.0;
        let t58 = t35 * t56 + t25;
        let t59 = M_CBRTPI;
        let t60 = t2 * t59;
        let t61 = t9 * t9;
        let t63 = 1.0 / t10 / t61;
        let t65 = sigma0 + 2.0 * sigma1 + sigma2;
        let t69 = 1.0 + t60 * t63 * t65 / 48.0;
        let t70 = 1.0 / t58;
        let t71 = param_h * t70;
        let t72 = f64::powf(t69, t71);
        let tzk0 = t58 * t72;
        zk[ip] += tzk0;
        let t74 = t8 / t19;
        let t78 = t18 / t10;
        let t81 = t3 * t74 / 9.0 + 2.0 / 9.0 * t14 * t78;
        let t83 = 1.0 / t23;
        let t84 = param_ap * t81 * t83;
        let t89 = t26 * t74 / 9.0 + 2.0 / 9.0 * t29 * t78;
        let t91 = 1.0 / t32;
        let t93 = param_af * t89 * t91 - t84;
        let t94 = t93 * t56;
        let t95 = t35 * t53;
        let t96 = 1.0 / t43;
        let t97 = 1.0 / t61;
        let t98 = t36 * t97;
        let t99 = t37 - t98;
        let t102 = piecewise3::<f64>(t40, 0.0, 2.0 / 3.0 * t96 * t99);
        let t103 = 1.0 / t48;
        let t104 = -t99;
        let t107 = piecewise3::<f64>(t47, 0.0, 2.0 / 3.0 * t103 * t104);
        let t109 = t102 / 2.0 + t107 / 2.0;
        let t112 = -6.0 * t95 * t109 + t84 + t94;
        let t113 = t9 * t112;
        let t115 = t9 * t58;
        let t116 = t58 * t58;
        let t117 = 1.0 / t116;
        let t118 = param_h * t117;
        let t119 = f64::ln(t69);
        let t120 = t112 * t119;
        let t122 = t71 * t2;
        let t123 = t61 * t9;
        let t125 = 1.0 / t10 / t123;
        let t126 = t59 * t125;
        let t127 = 1.0 / t69;
        let t128 = t65 * t127;
        let t129 = t126 * t128;
        let t131 = 7.0 / 144.0 * t122 * t129;
        let t132 = -t118 * t120 - t131;
        let t133 = t72 * t132;
        let tvrho0 = t113 * t72 + t115 * t133 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t135 = -t37 - t98;
        let t138 = piecewise3::<f64>(t40, 0.0, 2.0 / 3.0 * t96 * t135);
        let t139 = -t135;
        let t142 = piecewise3::<f64>(t47, 0.0, 2.0 / 3.0 * t103 * t139);
        let t144 = t138 / 2.0 + t142 / 2.0;
        let t147 = -6.0 * t95 * t144 + t84 + t94;
        let t148 = t9 * t147;
        let t150 = t147 * t119;
        let t152 = -t118 * t150 - t131;
        let t153 = t72 * t152;
        let tvrho1 = t115 * t153 + t148 * t72 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t156 = 1.0 / t10 / t9;
        let t157 = t156 * t72;
        let t159 = t60 * t127;
        let t160 = t157 * param_h * t159;
        let tvsigma0 = t160 / 48.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = t160 / 24.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
