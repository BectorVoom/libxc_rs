//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2096/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2096(t17544: f64, t7618: f64, t17373: f64, t29040: f64, t17769: f64, t7624: f64, t104695: f64, t13142: f64, t17384: f64, t26867: f64, t26827: f64, t5362: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104756 = 0.57165357490759649296e-3_f64 * t7618 * t17544;
    let t104768 = 0.11433071498151929859e-2_f64 * t29040 * t17373;
    let t104770 = 0.3811023832717309953e-3_f64 * t7624 * t17769;
    let t104774 = t13142 * t104695;
    let t104793 = 0.3811023832717309953e-3_f64 * t26867 * t17384;
    let t104815 = 0.57165357490759649296e-3_f64 * t26827 * t5362;
    (t104756, t104768, t104770, t104774, t104793, t104815)
}
