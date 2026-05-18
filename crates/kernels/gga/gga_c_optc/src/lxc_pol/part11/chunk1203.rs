//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1203/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1203<F: Float>(t123: F, t5311: F, t18005: F, t44014: F, t4435: F, t15274: F, t4464: F, t45343: F, t1162: F, t17907: F, t2367: F, t18130: F) -> (F, F, F, F, F) {
    let t55555 = t123 * t5311;
    let t55561 = t4435 * t44014 * t18005;
    let t55598 = t4464 * t45343 * t15274;
    let t55605 = t1162 * t2367 * t17907;
    let t55613 = t1162 * t2367 * t18130;
    (t55555, t55561, t55598, t55605, t55613)
}
