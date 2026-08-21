//! GGA_C_TCA vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_tca.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_tca_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = pow_1_3(zeta_threshold);
        let t3 = t2 * t2;
        let t4 = piecewise3(1.0 <= zeta_threshold, t3, 1.0);
        let t5 = t4 * t4;
        let t6 = t5 * t4;
        let t7 = M_CBRT3;
        let t9 = pow_1_3(1.0 / M_PI);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t18 = 4.88827 + 0.79425925 * t10 * t12 / t13;
        let t19 = rmath::atan(t18);
        let t21 = -0.655868 * t19 + 0.897889;
        let t22 = t6 * t21;
        let t23 = t7 * t7;
        let t24 = t22 * t23;
        let t25 = 1.0 / t9;
        let t26 = t25 * t11;
        let t27 = M_CBRT6;
        let t28 = t27 * t27;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT2;
        let t34 = rmath::sqrt(sigma[ip]);
        let t35 = t33 * t34;
        let t37 = 1.0 / t13 / rho[ip];
        let t39 = t32 * t35 * t37;
        let t40 = rmath::pow(t39, 2.3);
        let t42 = 1.0 + 0.004712150703442276 * t40;
        let t43 = 1.0 / t42;
        let t46 = t24 * t26 * t13 * t43;
        let tzk0 = t46 / 3.0;
        zk[ip] += tzk0;
        let t48 = t18 * t18;
        let t49 = t48 + 1.0;
        let t50 = 1.0 / t49;
        let t51 = t6 * t50;
        let t55 = 1.0 / rho[ip] * t6;
        let t57 = t23 * t25;
        let t58 = t57 * t11;
        let t60 = t42 * t42;
        let t61 = 1.0 / t60;
        let t62 = rmath::pow(t39, 1.3);
        let t63 = t61 * t62;
        let t64 = t63 * t28;
        let t65 = t31 * t33;
        let t66 = t65 * t34;
        let t67 = t64 * t66;
        let tvrho0 = 4.0 / 9.0 * t46 + 0.6945723010386666 * t51 * t43 + 0.004816865163518771 * t55 * t21 * t58 * t67;
        vrho[ip] += tvrho0;
        let t70 = t22 * t58;
        let t71 = 1.0 / t34;
        let t72 = t65 * t71;
        let tvsigma0 = -0.001806324436319539 * t70 * t64 * t72;
        vsigma[ip] += tvsigma0;
    }
}
