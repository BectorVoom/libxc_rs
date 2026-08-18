//! GGA_K_LC94 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lc94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lc94_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
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
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = param_alpha * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t44 = t38 * t43;
        let t47 = f64::exp(-t33 * t44 / 24.0);
        let t50 = (param_d * t47 + param_c) * t32;
        let t53 = t32 * t32;
        let t54 = 1.0 / t35;
        let t55 = t53 * t54;
        let t56 = f64::sqrt(sigma0);
        let t58 = 1.0 / t40 / rho0;
        let t62 = f64::powf(t55 * t56 * t58 / 12.0, param_expo);
        let t63 = param_f * t62;
        let t64 = t50 * t44 / 24.0 - t63;
        let t65 = t55 * t56;
        let t67 = param_b * t53;
        let t72 = f64::ln(t67 * t54 * t56 * t58 / 12.0 + f64::sqrt(pow_2(t67 * t54 * t56 * t58 / 12.0) + 1.0));
        let t73 = t58 * param_a * t72;
        let t76 = 1.0 + t65 * t73 / 12.0 + t63;
        let t77 = 1.0 / t76;
        let t79 = t64 * t77 + 1.0;
        let t83 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t79);
        let t84 = rho1 <= dens_threshold;
        let t85 = -t17;
        let t87 = piecewise5(t15, t12, t11, t16, t85 * t8);
        let t88 = 1.0 + t87;
        let t89 = t88 <= zeta_threshold;
        let t90 = pow_1_3(t88);
        let t91 = t90 * t90;
        let t93 = piecewise3(t89, t24, t91 * t88);
        let t94 = t93 * t30;
        let t95 = t37 * sigma2;
        let t96 = rho1 * rho1;
        let t97 = pow_1_3(rho1);
        let t98 = t97 * t97;
        let t100 = 1.0 / t98 / t96;
        let t101 = t95 * t100;
        let t104 = f64::exp(-t33 * t101 / 24.0);
        let t107 = (param_d * t104 + param_c) * t32;
        let t110 = f64::sqrt(sigma2);
        let t112 = 1.0 / t97 / rho1;
        let t116 = f64::powf(t55 * t110 * t112 / 12.0, param_expo);
        let t117 = param_f * t116;
        let t118 = t107 * t101 / 24.0 - t117;
        let t119 = t55 * t110;
        let t125 = f64::ln(t67 * t54 * t110 * t112 / 12.0 + f64::sqrt(pow_2(t67 * t54 * t110 * t112 / 12.0) + 1.0));
        let t126 = t112 * param_a * t125;
        let t129 = 1.0 + t119 * t126 / 12.0 + t117;
        let t130 = 1.0 / t129;
        let t132 = t118 * t130 + 1.0;
        let t136 = piecewise3(t84, 0.0, 3.0 / 20.0 * t6 * t94 * t132);
        let tzk0 = t83 + t136;
        zk[ip] += tzk0;
    }
}
