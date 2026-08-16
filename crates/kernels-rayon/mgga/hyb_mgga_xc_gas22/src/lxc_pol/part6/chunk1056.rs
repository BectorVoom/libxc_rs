//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1056/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1056(t10013: f64, t10022: f64, t10073: f64, t1211: f64, t1223: f64, t1959: f64, t3068: f64, t3072: f64, t3087: f64, t3105: f64, t3876: f64, t3881: f64, t3898: f64, t3909: f64, t6096: f64, t616: f64, t618: f64, t632: f64, t72: f64, t8080: f64, t8102: f64, t85: f64, t9999: f64) -> f64 {
    let t10076 = 7.0_f64 / 2.0_f64 * t3898 * t3087 - t8102 * t8080 - t10013 * t3087 / 4.0_f64 - 6.0_f64 * t6096 * t3881 * t616 + 4.0_f64 * t1959 * t1211 * t3068 - t3072 * t10022 / 2.0_f64 + 2.0_f64 * t1959 * t3876 * t616 - t618 * t9999 + 2.0_f64 * t9999 * t85 + 2.0_f64 * t3876 * t632 + 4.0_f64 * t3068 * t1223 + 4.0_f64 * t1211 * t3105 + 2.0_f64 * t616 * t3909 + 2.0_f64 * t72 * t10073;
    t10076
}
