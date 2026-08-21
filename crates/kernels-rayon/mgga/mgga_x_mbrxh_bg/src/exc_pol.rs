//! MGGA_X_MBRXH_BG exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbrxh_bg.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::br89::{xc_mgga_x_br89_get_x};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbrxh_bg_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
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
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = pow_1_3(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = pow_1_3(t16);
        let t22 = piecewise3(t17, t19, t20 * t16);
        let t23 = pow_1_3(t3);
        let t24 = t22 * t23;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = t24 * t27;
        let t29 = M_CBRT4;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t33 = 1.0 / t31 / rho0;
        let t36 = M_CBRT6;
        let t37 = t36 * t36;
        let t38 = M_PI * M_PI;
        let t39 = pow_1_3(t38);
        let t40 = t39 * t39;
        let t42 = 3.0 / 10.0 * t37 * t40;
        let t43 = rho0 * rho0;
        let t45 = 1.0 / t31 / t43;
        let t48 = sigma0 * sigma0;
        let t49 = t43 * t43;
        let t50 = t49 * rho0;
        let t52 = 1.0 / t30 / t50;
        let t55 = 0.46864 * tau0 * t33 - t42 + 0.089 * sigma0 * t45 + 0.0053 * t48 * t52;
        let t56 = rmath::abs(t55);
        let t57 = t56 < 5e-13;
        let t58 = 0.0 < t55;
        let t59 = piecewise3(t58, 5e-13, -5e-13);
        let t60 = piecewise3(t57, t59, t55);
        let t61 = xc_mgga_x_br89_get_x(t60);
        let t63 = rmath::exp(t61 / 3.0);
        let t64 = t29 * t63;
        let t65 = rmath::exp(-t61);
        let t67 = 1.0 + t61 / 2.0;
        let t68 = t65 * t67;
        let t69 = 1.0 - t68;
        let t70 = 1.0 / t61;
        let t71 = t69 * t70;
        let t72 = t64 * t71;
        let t75 = piecewise3(t2, 0.0, -t28 * t72 / 4.0);
        let t76 = rho1 <= dens_threshold;
        let t77 = -t13;
        let t79 = piecewise5(t11, t8, t7, t12, t77 * t4);
        let t80 = 1.0 + t79;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t19, t82 * t80);
        let t85 = t84 * t23;
        let t86 = t85 * t27;
        let t87 = pow_1_3(rho1);
        let t88 = t87 * t87;
        let t90 = 1.0 / t88 / rho1;
        let t93 = rho1 * rho1;
        let t95 = 1.0 / t88 / t93;
        let t98 = sigma2 * sigma2;
        let t99 = t93 * t93;
        let t100 = t99 * rho1;
        let t102 = 1.0 / t87 / t100;
        let t105 = 0.46864 * tau1 * t90 - t42 + 0.089 * sigma2 * t95 + 0.0053 * t98 * t102;
        let t106 = rmath::abs(t105);
        let t107 = t106 < 5e-13;
        let t108 = 0.0 < t105;
        let t109 = piecewise3(t108, 5e-13, -5e-13);
        let t110 = piecewise3(t107, t109, t105);
        let t111 = xc_mgga_x_br89_get_x(t110);
        let t113 = rmath::exp(t111 / 3.0);
        let t114 = t29 * t113;
        let t115 = rmath::exp(-t111);
        let t117 = 1.0 + t111 / 2.0;
        let t118 = t115 * t117;
        let t119 = 1.0 - t118;
        let t120 = 1.0 / t111;
        let t121 = t119 * t120;
        let t122 = t114 * t121;
        let t125 = piecewise3(t76, 0.0, -t86 * t122 / 4.0);
        let tzk0 = t75 + t125;
        zk[ip] += tzk0;
    }
}
