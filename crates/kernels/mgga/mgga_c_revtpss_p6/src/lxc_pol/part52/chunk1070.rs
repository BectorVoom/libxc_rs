//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1070/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1070<F: Float>(t32698: F, t32732: F, t532: F, t1450: F, t2014: F, t1353: F, t2033: F, t26405: F, t25082: F, t1453: F, t1932: F, t2089: F, t32107: F, t32109: F, t32112: F, t32660: F, t32663: F, t32667: F, t32671: F, t569: F, t6983: F, t7474: F, t7489: F, t8463: F, t8568: F, t8695: F) -> (F, F, F, F, F, F) {
    let t32733 = t32698 + t32732;
    let t32734 = t532 * t32733;
    let t32735 = t32734 * t1450;
    let t32736 = t2014 * t32735;
    let t32737 = t2033 * t1353;
    let t32738 = t26405 * t32737;
    let t32740 = F::cast_from(3.0_f64) * t25082 * t32738;
    let t32741 = t1453 * t8695 - t1932 * t7474 - t2089 * t6983 + t32660 * t569 + F::cast_from(3.0_f64) * t7489 * t8568 - t32107 - t32109 - t32112 - t32663 + t32667 + t32671 + t32736 - t32740 - t8463;
    (t32733, t32734, t32735, t32737, t32738, t32741)
}
