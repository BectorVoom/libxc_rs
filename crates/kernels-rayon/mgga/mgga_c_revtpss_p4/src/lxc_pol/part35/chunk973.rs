//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 973/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk973(t30: f64, t265: f64, t393: f64, t23436: f64, t23560: f64, t24190: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t22670: f64, t22671: f64, t395: f64, t45: f64, t5824: f64, t5825: f64, t6084: f64, t6405: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t24192 = piecewise3(t394, t23560 + t24190, t23436);
    let t24202 = piecewise3(t120, t23436 * t30 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t6084 * t1468 + 3.0_f64 / 2.0_f64 * t1587 * t5824 + t265 * t22670 / 2.0_f64, t24192 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t6405 * t1469 + 3.0_f64 / 2.0_f64 * t1704 * t5825 + t395 * t22671 / 2.0_f64);
    t24202
}
