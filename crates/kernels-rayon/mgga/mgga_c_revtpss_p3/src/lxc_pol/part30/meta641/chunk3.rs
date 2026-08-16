//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2231/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2231(t5326: f64, t7623: f64, t17544: f64, t7618: f64, t17523: f64, t26842: f64, t3594: f64, t7616: f64, t17373: f64, t29040: f64, t17769: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104752 = t5326 * t7623;
    let t104756 = 0.57165357490759649296e-3_f64 * t7618 * t17544;
    let t104758 = t3594 * t26842 * t17523;
    let t104762 = t3594 * t7616 * t17523;
    let t104768 = 0.11433071498151929859e-2_f64 * t29040 * t17373;
    let t104770 = 0.3811023832717309953e-3_f64 * t7624 * t17769;
    (t104752, t104756, t104758, t104762, t104768, t104770)
}
