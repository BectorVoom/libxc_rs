//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3525/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3525(t11921: f64, t19399: f64, t247: f64, t4837: f64, t15752: f64, t19741: f64, t3091: f64, t43240: f64, t6267: f64, t16088: f64, t380: f64, t4746: f64) -> (f64, f64, f64, f64) {
    let t66752 = t4837 * t247 * t11921 * t19399;
    let t66758 = t19741 * t15752;
    let t66763 = t3091 * t43240 * t6267;
    let t66766 = t4746 * t380 * t16088;
    (t66752, t66758, t66763, t66766)
}
