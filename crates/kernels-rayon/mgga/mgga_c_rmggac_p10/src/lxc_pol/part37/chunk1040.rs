//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1040/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1040(t73795: f64, t73801: f64, t76648: f64, t76652: f64, t76656: f64, t76658: f64, t76662: f64, t76666: f64, t76668: f64, t76670: f64, t76671: f64, t76673: f64, t76674: f64, t76679: f64, t76682: f64, t76683: f64, t76684: f64) -> f64 {
    let t79980 = -t76648 - t76652 - t76656 + t76658 + t76662 + t76666 + t76668 - t76670 + t76671 + 0.87596530464506835932e-6_f64 * t73795 - t76673 + t76674 + 0.87596530464506835932e-6_f64 * t73801 - t76679 + t76682 - t76683 - t76684;
    t79980
}
