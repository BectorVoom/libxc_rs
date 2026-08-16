//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1302/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1302(t530: f64, t7311: f64, t2014: f64, t25865: f64, t47672: f64, t9590: f64, t2034: f64, t13625: f64, t25082: f64, t32113: f64, t26088: f64, t531: f64) -> (f64, f64, f64, f64) {
    let t94345 = t530 * t7311;
    let t94348 = 18.0_f64 * t2014 * t94345 * t25865;
    let t94349 = t47672 * t9590;
    let t94352 = 6.0_f64 * t2014 * t2034 * t94349;
    let t94355 = 18.0_f64 * t25082 * t32113 * t13625;
    let t94358 = t531 * t26088;
    (t94348, t94352, t94355, t94358)
}
