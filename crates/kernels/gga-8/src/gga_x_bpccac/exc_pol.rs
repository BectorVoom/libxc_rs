//! GGA_X_BPCCAC exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 97 shared lines across all orders.
//! Delta: 97 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_bpccac_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
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
        // --- shared preamble (97 lines) ---
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
        let t28 = f64::sqrt(sigma0);
        let t29 = pow_1_3(rho0);
        let t31 = 1.0 / t29 / rho0;
        let t32 = t28 * t31;
        let t34 = f64::exp(-t32 + 19.0);
        let t35 = 1.0 + t34;
        let t36 = 1.0 / t35;
        let t37 = 1.0 - t36;
        let t38 = M_CBRT6;
        let t39 = M_PI * M_PI;
        let t40 = pow_1_3(t39);
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t38 * t42;
        let t44 = rho0 * rho0;
        let t45 = t29 * t29;
        let t47 = 1.0 / t45 / t44;
        let t49 = t43 * sigma0 * t47;
        let t51 = 0.1227e1 + 0.91464571985215458336e-2 * t49;
        let t54 = 0.2227e1 - 0.1505529e1 / t51;
        let t57 = f64::exp(-25.0 / 6.0 * t49);
        let t60 = (0.2743e0 - 0.1508e0 * t57) * t38;
        let t61 = t42 * sigma0;
        let t65 = t38 * t38;
        let t67 = 1.0 / t40 / t39;
        let t68 = t65 * t67;
        let t69 = sigma0 * sigma0;
        let t70 = t44 * t44;
        let t71 = t70 * rho0;
        let t73 = 1.0 / t29 / t71;
        let t76 = 0.69444444444444444444e-5 * t68 * t69 * t73;
        let t77 = t60 * t61 * t47 / 24.0 - t76;
        let t79 = t65 / t40;
        let t82 = f64::ln(0.64963333333333333333e0 * t79 * t32 + f64::sqrt(pow_2(0.64963333333333333333e0 * t79 * t32) + 1.0));
        let t86 = 1.0 + 0.16370833333333333333e-1 * t79 * t32 * t82 + t76;
        let t87 = 1.0 / t86;
        let t89 = t77 * t87 + 1.0;
        let t91 = t36 * t89 + t37 * t54;
        let t95 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t91);
        let t96 = rho1 <= dens_threshold;
        let t97 = -t16;
        let t99 = piecewise5(t14, t11, t10, t15, t97 * t7);
        let t100 = 1.0 + t99;
        let t101 = t100 <= zeta_threshold;
        let t102 = pow_1_3(t100);
        let t104 = piecewise3(t101, t22, t102 * t100);
        let t105 = t104 * t26;
        let t106 = f64::sqrt(sigma2);
        let t107 = pow_1_3(rho1);
        let t109 = 1.0 / t107 / rho1;
        let t110 = t106 * t109;
        let t112 = f64::exp(-t110 + 19.0);
        let t113 = 1.0 + t112;
        let t114 = 1.0 / t113;
        let t115 = 1.0 - t114;
        let t116 = rho1 * rho1;
        let t117 = t107 * t107;
        let t119 = 1.0 / t117 / t116;
        let t121 = t43 * sigma2 * t119;
        let t123 = 0.1227e1 + 0.91464571985215458336e-2 * t121;
        let t126 = 0.2227e1 - 0.1505529e1 / t123;
        let t129 = f64::exp(-25.0 / 6.0 * t121);
        let t132 = (0.2743e0 - 0.1508e0 * t129) * t38;
        let t133 = t42 * sigma2;
        let t137 = sigma2 * sigma2;
        let t138 = t116 * t116;
        let t139 = t138 * rho1;
        let t141 = 1.0 / t107 / t139;
        let t144 = 0.69444444444444444444e-5 * t68 * t137 * t141;
        let t145 = t132 * t133 * t119 / 24.0 - t144;
        let t148 = f64::ln(0.64963333333333333333e0 * t79 * t110 + f64::sqrt(pow_2(0.64963333333333333333e0 * t79 * t110) + 1.0));
        let t152 = 1.0 + 0.16370833333333333333e-1 * t79 * t110 * t148 + t144;
        let t153 = 1.0 / t152;
        let t155 = t145 * t153 + 1.0;
        let t157 = t114 * t155 + t115 * t126;
        let t161 = piecewise3(t96, 0.0, -3.0 / 8.0 * t5 * t105 * t157);
        let tzk0 = t95 + t161;
        zk[ip] += tzk0;
    }
}
