//! GGA_X_C09X vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_c09x.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_c09x_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = sigma[ip] * t28;
        let t36 = t25 * t34 * t32;
        let t38 = f64::exp(-0.0020125 * t36);
        let t39 = t33 * t38;
        let t43 = f64::exp(-0.00100625 * t36);
        let t45 = 2.245 + 0.0025708333333333334 * t26 * t39 - 1.245 * t43;
        let t49 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t45);
        let tzk0 = 2.0 * t49;
        zk[ip] += tzk0;
        let t51 = t17 / t30;
        let t55 = t29 * rho[ip];
        let t57 = 1.0 / t30 / t55;
        let t58 = t28 * t57;
        let t59 = t58 * t38;
        let t62 = t20 * t20;
        let t64 = 1.0 / t22 / t21;
        let t65 = t62 * t64;
        let t66 = sigma[ip] * sigma[ip];
        let t67 = t65 * t66;
        let t68 = t29 * t29;
        let t69 = t68 * t29;
        let t71 = 1.0 / t18 / t69;
        let t72 = t27 * t71;
        let t73 = t72 * t38;
        let t76 = t58 * t43;
        let t79 = -0.006855555555555556 * t26 * t59 + 2.7593611111111112e-05 * t67 * t73 - 0.00334075 * t26 * t76;
        let t84 = piecewise3(t2, 0.0, -t6 * t51 * t45 / 8.0 - 3.0 / 8.0 * t6 * t19 * t79);
        let tvrho0 = 2.0 * rho[ip] * t84 + 2.0 * t49;
        vrho[ip] += tvrho0;
        let t90 = t68 * rho[ip];
        let t93 = t27 / t18 / t90;
        let t94 = t93 * t38;
        let t100 = 0.0025708333333333334 * t25 * t39 - 1.0347604166666667e-05 * t65 * sigma[ip] * t94 + 0.00125278125 * t25 * t33 * t43;
        let t104 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t100);
        let tvsigma0 = 2.0 * rho[ip] * t104;
        vsigma[ip] += tvsigma0;
    }
}
