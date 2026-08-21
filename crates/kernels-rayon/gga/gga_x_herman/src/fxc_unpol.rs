//! GGA_X_HERMAN fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_herman.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_herman_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        let t20 = t3 * t3;
        let t22 = pow_1_3(1.0 / M_PI);
        let t23 = 1.0 / t22;
        let t25 = M_CBRT4;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t37 = 1.0 + 0.0006666666666666666 * t20 * t23 * t25 * sigma[ip] * t28 * t33;
        let t41 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
        let t47 = t30 * rho[ip];
        let t50 = t17 / t18 / t47;
        let t53 = t25 * sigma[ip] * t28;
        let t57 = piecewise3(t2, 0.0, -t6 * t17 / t31 * t37 / 8.0 + 0.0013655681265105914 * t50 * t23 * t53);
        let tvrho0 = 2.0 * rho[ip] * t57 + 2.0 * t41;
        vrho[ip] += tvrho0;
        let t64 = t23 * t25 * t28;
        let t67 = piecewise3(t2, 0.0, -0.0005120880474414717 * t17 / t18 / t30 * t64);
        let tvsigma0 = 2.0 * rho[ip] * t67;
        vsigma[ip] += tvsigma0;
        let t76 = t30 * t30;
        let t79 = t17 / t18 / t76;
        let t84 = piecewise3(t2, 0.0, t6 * t17 / t31 / rho[ip] * t37 / 12.0 - 0.004096704379531774 * t79 * t23 * t53);
        let tv2rho20 = 2.0 * rho[ip] * t84 + 4.0 * t57;
        v2rho2[ip] += tv2rho20;
        let t89 = piecewise3(t2, 0.0, 0.0011948721106967675 * t50 * t64);
        let tv2rhosigma0 = 2.0 * rho[ip] * t89 + 2.0 * t67;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
    }
}
