//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2085/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2085(t15604: f64, t4893: f64, t3117: f64, t4894: f64, t999: f64, t4583: f64, t4786: f64, t3092: f64, t3090: f64, t4954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15605 = t4893 * t15604;
    let t15606 = t3117 * t15605;
    let t15609 = t4894 * t999;
    let t15610 = t4893 * t15609;
    let t15611 = t3117 * t15610;
    let t15614 = t4583 * t4786;
    let t15615 = t3092 * t15614;
    let t15618 = t4954 * t3090;
    (t15605, t15606, t15609, t15610, t15611, t15614, t15615, t15618)
}
