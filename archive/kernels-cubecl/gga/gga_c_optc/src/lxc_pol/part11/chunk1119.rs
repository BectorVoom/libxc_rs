//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1119/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1119<F: Float>(t2885: F, t5454: F, t1220: F, t5097: F, t7274: F, t2838: F, t490: F, t5440: F, t11927: F, t4536: F, t5102: F, t16287: F, t172: F) -> (F, F, F, F, F, F) {
    let t47331 = t5454 * t2885;
    let t47639 = t1220 * t7274 * t5097;
    let t47654 = t490 * t5440 * t2838;
    let t47659 = t4536 * t11927;
    let t47709 = t1220 * t7274 * t5102;
    let t47744 = t172 * t16287;
    (t47331, t47639, t47654, t47659, t47709, t47744)
}
