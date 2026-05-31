//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1326/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1326<F: Float>(t28916: F, t789: F, t24556: F, t24559: F, t24562: F, t24658: F, t24661: F, t24664: F, t24667: F, t24670: F, t24673: F, t28907: F, t28917: F) -> (F, F) {
    let t28919 = t789 * t28916;
    let t28930 = F::cast_from(0.776775e1_f64) * t28907 + F::cast_from(0.16504875e0_f64) * t28917 + F::cast_from(0.258925e1_f64) * t28919 - F::cast_from(0.18786444444444444444e1_f64) * t24556 + F::cast_from(0.16102666666666666667e1_f64) * t24559 - F::cast_from(0.60385e0_f64) * t24562 + F::cast_from(0.11038e1_f64) * t24658 + F::cast_from(0.11038e1_f64) * t24661 - F::cast_from(0.14717333333333333333e1_f64) * t24664 - F::cast_from(0.33114e0_f64) * t24667 - F::cast_from(0.66228e0_f64) * t24670 - F::cast_from(0.33114e0_f64) * t24673;
    (t28919, t28930)
}
