//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1068/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1068(t32698: f64, t32732: f64, t532: f64, t1450: f64, t2014: f64, t1353: f64, t2033: f64, t26405: f64, t25082: f64, t2042: f64, t7547: f64, t2113: f64, t7331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32733 = t32698 + t32732;
    let t32734 = t532 * t32733;
    let t32735 = t32734 * t1450;
    let t32736 = t2014 * t32735;
    let t32737 = t2033 * t1353;
    let t32738 = t26405 * t32737;
    let t32740 = 3.0_f64 * t25082 * t32738;
    let t32760 = 3.0_f64 * t7547 * t2042;
    let t32762 = 6.0_f64 * t2113 * t7331;
    (t32733, t32734, t32735, t32736, t32737, t32738, t32740, t32760, t32762)
}
