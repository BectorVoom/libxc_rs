//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1482/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1482(t10613: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t14433: f64, t14618: f64, t18571: f64, t18572: f64, t18573: f64, t18574: f64, t18578: f64, t18579: f64, t18581: f64, t9524: f64, t9542: f64) -> (f64, f64) {
    let t18582 = 4.0_f64 * t10613;
    let t18583 = t14433 + t18571 - t9524 + t10592 + t18572 - t18573 - t10596 - t18574 + t18578 - t10604 + t9542 - t14618 + t18579 + t18581 - t10611 + t18582;
    (t18582, t18583)
}
