//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1276/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1276<F: Float>(t10057: F, t10073: F, t1211: F, t1223: F, t1947: F, t1954: F, t1959: F, t1987: F, t23453: F, t27443: F, t27539: F, t27564: F, t3068: F, t3076: F, t3093: F, t3105: F, t3876: F, t3898: F, t3909: F, t6088: F, t6096: F, t616: F, t618: F, t632: F, t8061: F, t81: F, t8103: F, t8138: F, t85: F, t9999: F) -> F {
    let t27607 = F::cast_from(2.0_f64) * t1947 * t3909 + F::cast_from(4.0_f64) * t616 * t10073 + F::cast_from(4.0_f64) * t9999 * t632 + F::cast_from(2.0_f64) * t3876 * t1987 + F::cast_from(4.0_f64) * t8061 * t1223 + F::cast_from(8.0_f64) * t3068 * t3105 + F::cast_from(4.0_f64) * t1211 * t8138 + F::cast_from(2.0_f64) * t27443 * t85 - t618 * t27443 - t1954 * t27539 * t81 + F::cast_from(4.0_f64) * t1959 * t27539 + F::cast_from(14.0_f64) * t3093 * t27564 - t23453 * t27564 - F::cast_from(24.0_f64) * t6096 * t3076 * t3068 + F::cast_from(7.0_f64) / F::cast_from(2.0_f64) * t3898 * t6088 + F::cast_from(15.0_f64) / F::cast_from(4.0_f64) * t10057 * t8103;
    t27607
}
