//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1276/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1276(t10057: f64, t10073: f64, t1211: f64, t1223: f64, t1947: f64, t1954: f64, t1959: f64, t1987: f64, t23453: f64, t27443: f64, t27539: f64, t27564: f64, t3068: f64, t3076: f64, t3093: f64, t3105: f64, t3876: f64, t3898: f64, t3909: f64, t6088: f64, t6096: f64, t616: f64, t618: f64, t632: f64, t8061: f64, t81: f64, t8103: f64, t8138: f64, t85: f64, t9999: f64) -> f64 {
    let t27607 = 2.0_f64 * t1947 * t3909 + 4.0_f64 * t616 * t10073 + 4.0_f64 * t9999 * t632 + 2.0_f64 * t3876 * t1987 + 4.0_f64 * t8061 * t1223 + 8.0_f64 * t3068 * t3105 + 4.0_f64 * t1211 * t8138 + 2.0_f64 * t27443 * t85 - t618 * t27443 - t1954 * t27539 * t81 + 4.0_f64 * t1959 * t27539 + 14.0_f64 * t3093 * t27564 - t23453 * t27564 - 24.0_f64 * t6096 * t3076 * t3068 + 7.0_f64 / 2.0_f64 * t3898 * t6088 + 15.0_f64 / 4.0_f64 * t10057 * t8103;
    t27607
}
