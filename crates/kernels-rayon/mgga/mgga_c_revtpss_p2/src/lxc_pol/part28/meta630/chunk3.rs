//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2275/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2275(t1455: f64, t7956: f64, t1464: f64, t7939: f64, t2037: f64, t5808: f64, t1921: f64, t7318: f64, t2045: f64, t5789: f64, t18178: f64, t18217: f64, t2038: f64, t28235: f64, t4154: f64, t5790: f64, t7337: f64, t92556: f64, t95125: f64, t95180: f64) -> f64 {
    let t101661 = 2.0_f64 * t1455 * t7956;
    let t101668 = 2.0_f64 * t7939 * t1464;
    let t101670 = 2.0_f64 * t2037 * t5808;
    let t101672 = 2.0_f64 * t7318 * t1921;
    let t101674 = 2.0_f64 * t5789 * t2045;
    let t101678 = 2.0_f64 * t1464 * t28235 + t18178 * t2045 + t18217 * t2038 + t4154 * t7956 + 2.0_f64 * t5790 * t7337 + t101661 + t101668 + t101670 + t101672 + t101674 + t92556 + 2.0_f64 * t95125 + t95180;
    t101678
}
