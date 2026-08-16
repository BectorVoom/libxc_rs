//! GGA_X_LG93 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lg93.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lg93_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t6 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t19 * t19;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = t20 * t20;
        let t38 = 1.0 / t22 / t21;
        let t39 = t36 * t38;
        let t40 = sigma[ip] * sigma[ip];
        let t41 = t40 * t26;
        let t42 = t29 * t29;
        let t43 = t42 * rho[ip];
        let t45 = 1.0 / t19 / t43;
        let t49 = t40 * sigma[ip];
        let t50 = t42 * t42;
        let t51 = 1.0 / t50;
        let t54 = t21 * t21;
        let t57 = t20 / t23 / t54;
        let t58 = t40 * t40;
        let t59 = t58 * t27;
        let t60 = t50 * t29;
        let t62 = 1.0 / t30 / t60;
        let t69 = t36 / t22 / t54 / t21;
        let t70 = t58 * sigma[ip];
        let t71 = t70 * t26;
        let t72 = t50 * t43;
        let t74 = 1.0 / t19 / t72;
        let t78 = t58 * t40;
        let t79 = t50 * t50;
        let t80 = 1.0 / t79;
        let t83 = 1.0 + 0.20588079936467259283e0 * t34 + 0.1034375e0 * t39 * t41 * t45 + 0.39953563229732420473e-3 * t49 * t51 + 0.87666377314814814812e-3 * t57 * t59 * t62 + 0.9464819637345679012e-2 * t69 * t71 * t74 + 0.17770905884280507538e-7 * t78 * t80;
        let t84 = f64::powf(t83, 0.24974e-1);
        let t87 = 1.0 + 0.41666666666666666666e-9 * t34;
        let t88 = 1.0 / t87;
        let t92 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t84 * t88);
        let tzk0 = 2.0 * t92;
        zk[ip] += tzk0;
    }
}
