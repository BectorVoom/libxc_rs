//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 627/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk627(t15881: f64, t338: f64, t118: f64, t14354: f64, t15175: f64, t15564: f64, t15566: f64, t15568: f64, t15571: f64, t15573: f64, t15574: f64, t15581: f64, t15584: f64) -> (f64, f64) {
    let t15882 = t338 * t15881;
    let t15883 = t118 * t15882;
    let t15885 = t15564 - t15566 - t15568 - t15571 - t15573 - t15175 + t15574 - t15581 + t15584 + t14354 + 0.19957069503106347607e-1_f64 * t15883;
    (t15882, t15885)
}
