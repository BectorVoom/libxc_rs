//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1192/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1192<F: Float>(t10057: F, t10073: F, t1211: F, t1223: F, t1947: F, t1954: F, t1959: F, t1987: F, t23453: F, t27443: F, t27539: F, t27564: F, t3068: F, t3076: F, t3093: F, t3105: F, t3876: F, t3898: F, t3909: F, t6088: F, t6096: F, t616: F, t618: F, t632: F, t8061: F, t81: F, t8103: F, t8138: F, t85: F, t9999: F) -> (F,) {
    let t27607 = 2.0 * t1947 * t3909 + 4.0 * t616 * t10073 + 4.0 * t9999 * t632 + 2.0 * t3876 * t1987 + 4.0 * t8061 * t1223 + 8.0 * t3068 * t3105 + 4.0 * t1211 * t8138 + 2.0 * t27443 * t85 - t618 * t27443 - t1954 * t27539 * t81 + 4.0 * t1959 * t27539 + 14.0 * t3093 * t27564 - t23453 * t27564 - 24.0 * t6096 * t3076 * t3068 + 7.0 / 2.0 * t3898 * t6088 + 15.0 / 4.0 * t10057 * t8103;
    (t27607,)
}
