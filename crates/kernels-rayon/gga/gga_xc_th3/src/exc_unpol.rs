//! GGA_XC_TH3 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th3.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th3_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_18: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rmath::pow(2.0, 1.0 / 6.0);
        let t3 = t2 * t2;
        let t4 = t3 * t3;
        let t6 = param_omega_0 * t4 * t2;
        let t7 = rmath::pow(rho[ip], 1.0 / 6.0);
        let t8 = t7 * rho[ip];
        let t12 = M_CBRT2;
        let t13 = t12 * t12;
        let t14 = param_omega_1 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = t15 * rho[ip];
        let t20 = M_SQRT2;
        let t21 = param_omega_2 * t20;
        let t22 = rmath::sqrt(rho[ip]);
        let t23 = t22 * rho[ip];
        let t27 = param_omega_3 * t12;
        let t28 = t15 * t15;
        let t29 = t28 * rho[ip];
        let t33 = rmath::pow(2.0, 1.0 / 12.0);
        let t34 = t33 * t33;
        let t36 = t34 * t34;
        let t38 = param_omega_4 * t36 * t34 * t33;
        let t39 = rmath::pow(rho[ip], 1.0 / 12.0);
        let t40 = rmath::sqrt(sigma[ip]);
        let t43 = pow_1_3(zeta_threshold);
        let t45 = piecewise3(1.0 <= zeta_threshold, t43 * zeta_threshold, 1.0);
        let t50 = param_omega_5 * t20;
        let t56 = param_omega_6 * t12;
        let t62 = param_omega_7 * t2;
        let t68 = param_omega_8 * t12;
        let t69 = 1.0 / rho[ip];
        let t71 = t45 * t45;
        let t76 = param_omega_9 * t2;
        let t77 = t7 * t7;
        let t78 = t77 * t77;
        let t79 = t78 * t7;
        let t80 = 1.0 / t79;
        let t85 = param_omega_10;
        let t86 = 1.0 / t28;
        let t87 = t85 * t86;
        let t88 = sigma[ip] * t71;
        let t92 = param_omega_11 * t12;
        let t93 = rho[ip] * rho[ip];
        let t95 = 1.0 / t28 / t93;
        let t96 = sigma[ip] * t95;
        let t98 = t96 * t71 - t96;
        let t103 = param_omega_12 * t2;
        let t104 = t79 * rho[ip];
        let t108 = param_omega_13;
        let t109 = t108 * t93;
        let t112 = param_omega_18;
        let t113 = rmath::pow(rho[ip], 1.0833333333333333);
        let t116 = t6 * t8 / 2.0 + t14 * t16 / 2.0 + t21 * t23 / 2.0 + t27 * t29 / 2.0 + t38 * t39 * t40 * t45 / 4.0 + t50 * t7 * t40 * t45 / 4.0 + t56 * t15 * t40 * t45 / 4.0 + t62 * t22 * t40 * t45 / 4.0 + t68 * t69 * sigma[ip] * t71 / 8.0 + t76 * t80 * sigma[ip] * t71 / 8.0 + t87 * t88 / 8.0 + t92 * t29 * t98 / 2.0 + t103 * t104 * t98 / 2.0 + t109 * t98 / 2.0 + 0.9438743126816935 * t112 * t113;
        let tzk0 = t116 * t69;
        zk[ip] += tzk0;
    }
}
