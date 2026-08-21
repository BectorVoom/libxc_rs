//! GGA_X_HCTH_A vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hcth_a.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hcth_a_vxc_unpol(
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
        let t20 = t3 * t3;
        let t22 = pow_1_3(1.0 / M_PI);
        let t25 = M_CBRT4;
        let t26 = t20 / t22 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = rmath::sqrt(sigma[ip]);
        let t35 = t34 * t27;
        let t37 = 1.0 / t18 / rho[ip];
        let t39 = rmath::ln(t35 * t37 + rmath::sqrt(pow_2(t35 * t37) + 1.0));
        let t40 = t37 * t39;
        let t43 = 1.0 + 0.0252 * t35 * t40;
        let t46 = t43 * t43;
        let t47 = 1.0 / t46;
        let t49 = -2.51173 / t43 + 3.7198333333333333 * t47;
        let t54 = 1.09878 + 0.0009333333333333333 * t26 * t29 * t33 * t49;
        let t58 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        let t60 = t17 / t31;
        let t64 = t30 * rho[ip];
        let t66 = 1.0 / t31 / t64;
        let t73 = 1.0 / t18 / t30 * t39;
        let t77 = t29 * t33 + 1.0;
        let t78 = rmath::sqrt(t77);
        let t79 = 1.0 / t78;
        let t80 = t66 * t79;
        let t83 = -0.0336 * t35 * t73 - 0.0336 * t29 * t80;
        let t87 = 1.0 / t46 / t43;
        let t88 = t87 * t83;
        let t90 = 2.51173 * t47 * t83 - 7.439666666666667 * t88;
        let t95 = -0.002488888888888889 * t26 * t29 * t66 * t49 + 0.0009333333333333333 * t26 * t29 * t33 * t90;
        let t100 = piecewise3(t2, 0.0, -t6 * t60 * t54 / 8.0 - 3.0 / 8.0 * t6 * t19 * t95);
        let tvrho0 = 2.0 * rho[ip] * t100 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t103 = t28 * t33;
        let t108 = 1.0 / t34 * t27;
        let t113 = 0.0126 * t108 * t40 + 0.0126 * t103 * t79;
        let t116 = t87 * t113;
        let t118 = 2.51173 * t47 * t113 - 7.439666666666667 * t116;
        let t123 = 0.0009333333333333333 * t26 * t103 * t49 + 0.0009333333333333333 * t26 * t29 * t33 * t118;
        let t127 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
    }
}
