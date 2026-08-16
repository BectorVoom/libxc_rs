//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2963/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2963<F: Float>(t17884: F, t3117: F, t18029: F, t3108: F, t1021: F, t1025: F, t10863: F, t10957: F, t10965: F, t1618: F, t17607: F, t248: F, t3043: F, t3057: F, t3064: F, t3098: F, t3130: F, t3131: F, t3134: F, t48446: F, t49678: F, t5857: F, t5861: F, t5900: F, t61719: F, t61731: F, t61736: F, t61739: F, t61742: F) -> F {
    let t61744 = t3117 * t17884;
    let t61754 = t18029 * t3108;
    let t61760 = t3130 * t248 * t1021 * t61719 * t3131 / F::cast_from(768.0_f64) + t17607 * t3057 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t17607 * t3064 + t61731 * t1025 / F::cast_from(1536.0_f64) + t61736 * t3134 / F::cast_from(1536.0_f64) - t61739 * t3043 / F::cast_from(3072.0_f64) + t61742 / F::cast_from(432.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t61744 + t10965 * t5857 / F::cast_from(4608.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t49678 * t1618 + t10863 * t5900 / F::cast_from(216.0_f64) + F::cast_from(95.0_f64) / F::cast_from(7776.0_f64) * t10957 * t5861 - t61754 * t1025 / F::cast_from(288.0_f64) - t17607 * t3098 / F::cast_from(2304.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t48446;
    t61760
}
