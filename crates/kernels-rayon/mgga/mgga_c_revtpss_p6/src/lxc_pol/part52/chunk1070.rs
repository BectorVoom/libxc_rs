//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1070/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1070(t32698: f64, t32732: f64, t532: f64, t1450: f64, t2014: f64, t1353: f64, t2033: f64, t26405: f64, t25082: f64, t1453: f64, t1932: f64, t2089: f64, t32107: f64, t32109: f64, t32112: f64, t32660: f64, t32663: f64, t32667: f64, t32671: f64, t569: f64, t6983: f64, t7474: f64, t7489: f64, t8463: f64, t8568: f64, t8695: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32733 = t32698 + t32732;
    let t32734 = t532 * t32733;
    let t32735 = t32734 * t1450;
    let t32736 = t2014 * t32735;
    let t32737 = t2033 * t1353;
    let t32738 = t26405 * t32737;
    let t32740 = 3.0_f64 * t25082 * t32738;
    let t32741 = t1453 * t8695 - t1932 * t7474 - t2089 * t6983 + t32660 * t569 + 3.0_f64 * t7489 * t8568 - t32107 - t32109 - t32112 - t32663 + t32667 + t32671 + t32736 - t32740 - t8463;
    (t32733, t32734, t32735, t32737, t32738, t32741)
}
