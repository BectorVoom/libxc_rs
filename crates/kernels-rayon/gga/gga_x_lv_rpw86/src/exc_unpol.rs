//! GGA_X_LV_RPW86 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lv_rpw86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lv_rpw86_exc_unpol(
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
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t25 = t20 / t23;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 1.0 + 0.003931018518518519 * t34;
        let t37 = sigma[ip] * sigma[ip];
        let t38 = t37 * sigma[ip];
        let t39 = t29 * t29;
        let t40 = t39 * t39;
        let t41 = 1.0 / t40;
        let t42 = t38 * t41;
        let t43 = 3.881824540052514e-07 * t42;
        let t44 = 1.0 + t43;
        let t45 = 1.0 / t44;
        let t48 = t20 * t20;
        let t51 = t48 / t22 / t21;
        let t52 = t37 * t26;
        let t53 = t39 * rho[ip];
        let t55 = 1.0 / t18 / t53;
        let t60 = 1.0 + 0.077125 * t34 + 0.06017361111111111 * t51 * t52 * t55 + 2.905130394988796e-06 * t42;
        let t61 = rmath::pow(t60, 1.0 / 15.0);
        let t62 = 1.15 + t43;
        let t63 = 1.0 / t62;
        let t64 = t61 * t63;
        let t67 = t36 * t45 + 3.881824540052514e-07 * t42 * t64;
        let t71 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
    }
}
