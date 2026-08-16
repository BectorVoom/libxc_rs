//! MGGA_C_CCALDA exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ccalda_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c: f64,
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
        let t2 = 1.0 + param_c;
        let t3 = pow_1_3(rho0);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / rho0;
        let t7 = tau0 * t6;
        let t8 = rho0 - rho1;
        let t9 = rho0 + rho1;
        let t10 = 1.0 / t9;
        let t11 = t8 * t10;
        let t12 = 1.0 + t11;
        let t13 = t12 / 2.0;
        let t14 = pow_1_3(t13);
        let t15 = t14 * t14;
        let t16 = t15 * t13;
        let t17 = t7 * t16;
        let t18 = pow_1_3(rho1);
        let t19 = t18 * t18;
        let t21 = 1.0 / t19 / rho1;
        let t22 = tau1 * t21;
        let t23 = 1.0 - t11;
        let t24 = t23 / 2.0;
        let t25 = pow_1_3(t24);
        let t26 = t25 * t25;
        let t27 = t26 * t24;
        let t28 = t22 * t27;
        let t30 = sigma0 + 2.0 * sigma1 + sigma2;
        let t31 = t9 * t9;
        let t32 = pow_1_3(t9);
        let t33 = t32 * t32;
        let t35 = 1.0 / t33 / t31;
        let t38 = t17 + t28 - t30 * t35 / 8.0;
        let t39 = t2 * t38;
        let t40 = M_CBRT6;
        let t41 = M_PI * M_PI;
        let t42 = pow_1_3(t41);
        let t43 = t42 * t42;
        let t44 = 1.0 / t43;
        let t45 = t40 * t44;
        let t46 = t39 * t45;
        let t47 = M_CBRT2;
        let t48 = t47 * t47;
        let t50 = t45 * t48;
        let t53 = 1.0 + 5.0 / 9.0 * param_c * t38 * t50;
        let t54 = 1.0 / t53;
        let t55 = t48 * t54;
        let t56 = t31 * t31;
        let t58 = 1.0 / t33 / t56;
        let t59 = t30 * t58;
        let t60 = t17 + t28;
        let t61 = 1.0 / t60;
        let t62 = t8 * t8;
        let t63 = t61 * t62;
        let t66 = 1.0 - t59 * t63 / 8.0;
        let t67 = M_CBRT3;
        let t68 = 1.0 / M_PI;
        let t69 = pow_1_3(t68);
        let t70 = t67 * t69;
        let t71 = M_CBRT4;
        let t72 = t71 * t71;
        let t75 = t70 * t72 / t32;
        let t77 = 1.0 + 0.53425e-1 * t75;
        let t78 = f64::sqrt(t75);
        let t81 = pow_3_2(t75);
        let t83 = t67 * t67;
        let t84 = t69 * t69;
        let t85 = t83 * t84;
        let t88 = t85 * t71 / t33;
        let t90 = 0.379785e1 * t78 + 0.8969e0 * t75 + 0.204775e0 * t81 + 0.123235e0 * t88;
        let t93 = 1.0 + 0.16081979498692535067e2 / t90;
        let t94 = f64::ln(t93);
        let t96 = 0.621814e-1 * t77 * t94;
        let t97 = t62 * t62;
        let t98 = 1.0 / t56;
        let t99 = t97 * t98;
        let t100 = t12 <= zeta_threshold;
        let t101 = pow_1_3(zeta_threshold);
        let t102 = t101 * zeta_threshold;
        let t103 = pow_1_3(t12);
        let t105 = piecewise3(t100, t102, t103 * t12);
        let t106 = t23 <= zeta_threshold;
        let t107 = pow_1_3(t23);
        let t109 = piecewise3(t106, t102, t107 * t23);
        let t110 = t105 + t109 - 2.0;
        let t113 = 1.0 / (2.0 * t47 - 2.0);
        let t114 = t110 * t113;
        let t116 = 1.0 + 0.5137e-1 * t75;
        let t121 = 0.705945e1 * t78 + 0.1549425e1 * t75 + 0.420775e0 * t81 + 0.1562925e0 * t88;
        let t124 = 1.0 + 0.32163958997385070134e2 / t121;
        let t125 = f64::ln(t124);
        let t129 = 1.0 + 0.278125e-1 * t75;
        let t134 = 0.51785e1 * t78 + 0.905775e0 * t75 + 0.1100325e0 * t81 + 0.1241775e0 * t88;
        let t137 = 1.0 + 0.29608749977793437516e2 / t134;
        let t138 = f64::ln(t137);
        let t139 = t129 * t138;
        let t141 = -0.310907e-1 * t116 * t125 + t96 - 0.19751673498613801407e-1 * t139;
        let t142 = t114 * t141;
        let t146 = -t96 + t99 * t142 + 0.19751673498613801407e-1 * t114 * t139;
        let t147 = t66 * t146;
        let t148 = t55 * t147;
        let t150 = 5.0 / 9.0 * t46 * t148;
        let t151 = t39 * t40;
        let t152 = t44 * t48;
        let t153 = t152 * t54;
        let t154 = t151 * t153;
        let t156 = 1.0 - 5.0 / 9.0 * t154;
        let t157 = t156 * t146;
        let tzk0 = t150 + t157;
        zk[ip] += tzk0;
    }
}
