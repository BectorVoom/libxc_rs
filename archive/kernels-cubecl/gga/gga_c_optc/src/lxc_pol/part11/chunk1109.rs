//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1109/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1109<F: Float>(t11781: F, t5328: F, t3200: F, t3212: F, t5393: F, t3137: F, t3192: F, t5412: F, t3217: F, t5398: F, t12594: F, t4492: F) -> (F, F, F, F, F) {
    let t45343 = t11781 * t5328;
    let t45418 = t3212 * t3200 * t5393;
    let t45421 = t3192 * t3137 * t5412;
    let t45424 = t3217 * t3200 * t5398;
    let t45430 = t4492 * t12594;
    (t45343, t45418, t45421, t45424, t45430)
}
