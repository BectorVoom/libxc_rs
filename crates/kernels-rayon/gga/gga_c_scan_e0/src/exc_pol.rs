//! GGA_C_SCAN_E0 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_scan_e0.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_scan_e0_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t11 = t4 * t6 / t8;
        let t13 = 1.0 + 0.53425e-1 * t11;
        let t14 = f64::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t1 * t1;
        let t20 = t3 * t3;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t5 / t22;
        let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
        let t30 = 1.0 + 0.16081979498692535067e2 / t27;
        let t31 = f64::ln(t30);
        let t33 = 0.621814e-1 * t13 * t31;
        let t34 = rho0 - rho1;
        let t35 = t34 * t34;
        let t36 = t35 * t35;
        let t37 = t7 * t7;
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t36 * t39;
        let t41 = 1.0 / t7;
        let t42 = t34 * t41;
        let t43 = 1.0 + t42;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(zeta_threshold);
        let t46 = t45 * zeta_threshold;
        let t47 = pow_1_3(t43);
        let t48 = t47 * t43;
        let t49 = piecewise3(t44, t46, t48);
        let t50 = 1.0 - t42;
        let t51 = t50 <= zeta_threshold;
        let t52 = pow_1_3(t50);
        let t53 = t52 * t50;
        let t54 = piecewise3(t51, t46, t53);
        let t55 = t49 + t54 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t55 * t59;
        let t62 = 1.0 + 0.5137e-1 * t11;
        let t67 = 0.705945e1 * t14 + 0.1549425e1 * t11 + 0.420775e0 * t17 + 0.1562925e0 * t25;
        let t70 = 1.0 + 0.32163958997385070134e2 / t67;
        let t71 = f64::ln(t70);
        let t75 = 1.0 + 0.278125e-1 * t11;
        let t80 = 0.51785e1 * t14 + 0.905775e0 * t11 + 0.1100325e0 * t17 + 0.1241775e0 * t25;
        let t83 = 1.0 + 0.29608749977793437516e2 / t80;
        let t84 = f64::ln(t83);
        let t85 = t75 * t84;
        let t87 = -0.310907e-1 * t62 * t71 + t33 - 0.19751673498613801407e-1 * t85;
        let t88 = t60 * t87;
        let t89 = t40 * t88;
        let t91 = 0.19751673498613801407e-1 * t60 * t85;
        let t92 = f64::ln(2.0);
        let t93 = 1.0 - t92;
        let t94 = M_PI * M_PI;
        let t96 = t93 / t94;
        let t97 = t45 * t45;
        let t98 = t47 * t47;
        let t99 = piecewise3(t44, t97, t98);
        let t100 = t52 * t52;
        let t101 = piecewise3(t51, t97, t100);
        let t103 = t99 / 2.0 + t101 / 2.0;
        let t104 = t103 * t103;
        let t105 = t104 * t103;
        let t107 = 1.0 + 0.25e-1 * t11;
        let t109 = 1.0 + 0.4445e-1 * t11;
        let t110 = 1.0 / t109;
        let t111 = t107 * t110;
        let t112 = 1.0 / t93;
        let t114 = (-t33 + t89 + t91) * t112;
        let t115 = 1.0 / t105;
        let t116 = t94 * t115;
        let t118 = f64::exp(-t114 * t116);
        let t119 = t118 - 1.0;
        let t120 = 1.0 / t119;
        let t121 = t112 * t120;
        let t123 = sigma0 + 2.0 * sigma1 + sigma2;
        let t124 = t121 * t123;
        let t125 = t111 * t124;
        let t127 = 1.0 / t8 / t37;
        let t128 = t127 * t56;
        let t129 = 1.0 / t104;
        let t131 = 1.0 / t3;
        let t132 = t19 * t131;
        let t133 = t132 * t5;
        let t137 = 1.0 + 0.27439371595564631661e-1 * t125 * t128 * t129 * t133;
        let t138 = pow_1_4(t137);
        let t140 = 1.0 - 1.0 / t138;
        let t143 = 1.0 + 1.0 * t140 * t119;
        let t144 = f64::ln(t143);
        let t146 = t96 * t105 * t144;
        let tzk0 = -t33 + t89 + t91 + t146;
        zk[ip] += tzk0;
    }
}
