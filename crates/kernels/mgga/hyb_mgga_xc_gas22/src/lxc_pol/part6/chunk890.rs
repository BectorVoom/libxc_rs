//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 890/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk890<F: Float>(t1211: F, t1223: F, t1947: F, t1955: F, t1959: F, t1976: F, t1987: F, t3068: F, t3072: F, t3073: F, t3105: F, t6096: F, t616: F, t618: F, t632: F, t72: F, t8061: F, t8074: F, t8077: F, t8080: F, t8138: F, t85: F) -> (F,) {
    let t8141 = 7.0 / 2.0 * t1976 * t3073 - t8074 * t3073 / 2.0 - t8077 * t3073 / 4.0 - t3072 * t8080 - 6.0 * t6096 * t1211 * t1955 + 4.0 * t1959 * t3068 * t616 + 2.0 * t1959 * t1211 * t1947 - t618 * t8061 + 2.0 * t8061 * t85 + 4.0 * t3068 * t632 + 2.0 * t1211 * t1987 + 2.0 * t1947 * t1223 + 4.0 * t616 * t3105 + 2.0 * t72 * t8138;
    (t8141,)
}
