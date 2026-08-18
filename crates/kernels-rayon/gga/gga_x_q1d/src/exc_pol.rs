//! GGA_X_Q1D exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q1d.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_q1d_exc_pol(
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
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = 0.804 + 5.0 / 972.0 * t40;
        let t44 = 0.646416 / t42;
        let t46 = t28 * t28;
        let t48 = 1.0 / t30 / t29;
        let t49 = t46 * t48;
        let t50 = sigma0 * sigma0;
        let t51 = t34 * t34;
        let t52 = t51 * rho0;
        let t54 = 1.0 / t35 / t52;
        let t57 = t49 * t50 * t54 / 576.0;
        let t58 = t40 / 24.0 + t57;
        let t59 = t29 * t29;
        let t60 = 1.0 / t59;
        let t61 = t50 * sigma0;
        let t62 = t60 * t61;
        let t63 = t51 * t51;
        let t64 = 1.0 / t63;
        let t67 = 1.0 + t57 + t62 * t64 / 2304.0;
        let t68 = 1.0 / t67;
        let t69 = t58 * t68;
        let t71 = (1.804 - t44) * t28;
        let t72 = t32 * sigma0;
        let t76 = -t71 * t72 * t38 / 24.0 + 0.06525;
        let t78 = 1.804 - t44 + t69 * t76;
        let t82 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t78);
        let t83 = rho1 <= dens_threshold;
        let t84 = -t16;
        let t86 = piecewise5(t14, t11, t10, t15, t84 * t7);
        let t87 = 1.0 + t86;
        let t88 = t87 <= zeta_threshold;
        let t89 = pow_1_3(t87);
        let t91 = piecewise3(t88, t22, t89 * t87);
        let t92 = t91 * t26;
        let t93 = rho1 * rho1;
        let t94 = pow_1_3(rho1);
        let t95 = t94 * t94;
        let t97 = 1.0 / t95 / t93;
        let t99 = t33 * sigma2 * t97;
        let t101 = 0.804 + 5.0 / 972.0 * t99;
        let t103 = 0.646416 / t101;
        let t105 = sigma2 * sigma2;
        let t106 = t93 * t93;
        let t107 = t106 * rho1;
        let t109 = 1.0 / t94 / t107;
        let t112 = t49 * t105 * t109 / 576.0;
        let t113 = t99 / 24.0 + t112;
        let t114 = t105 * sigma2;
        let t115 = t60 * t114;
        let t116 = t106 * t106;
        let t117 = 1.0 / t116;
        let t120 = 1.0 + t112 + t115 * t117 / 2304.0;
        let t121 = 1.0 / t120;
        let t122 = t113 * t121;
        let t124 = (1.804 - t103) * t28;
        let t125 = t32 * sigma2;
        let t129 = -t124 * t125 * t97 / 24.0 + 0.06525;
        let t131 = 1.804 - t103 + t122 * t129;
        let t135 = piecewise3(t83, 0.0, -3.0 / 8.0 * t5 * t92 * t131);
        let tzk0 = t82 + t135;
        zk[ip] += tzk0;
    }
}
