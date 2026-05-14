//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1013/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1013<F: Float>(t10013: F, t10022: F, t10073: F, t1211: F, t1223: F, t1959: F, t3068: F, t3072: F, t3087: F, t3105: F, t3876: F, t3881: F, t3898: F, t3909: F, t6096: F, t616: F, t618: F, t632: F, t72: F, t8080: F, t8102: F, t85: F, t9999: F) -> (F,) {
    let t10076 = 7.0 / 2.0 * t3898 * t3087 - t8102 * t8080 - t10013 * t3087 / 4.0 - 6.0 * t6096 * t3881 * t616 + 4.0 * t1959 * t1211 * t3068 - t3072 * t10022 / 2.0 + 2.0 * t1959 * t3876 * t616 - t618 * t9999 + 2.0 * t9999 * t85 + 2.0 * t3876 * t632 + 4.0 * t3068 * t1223 + 4.0 * t1211 * t3105 + 2.0 * t616 * t3909 + 2.0 * t72 * t10073;
    (t10076,)
}
