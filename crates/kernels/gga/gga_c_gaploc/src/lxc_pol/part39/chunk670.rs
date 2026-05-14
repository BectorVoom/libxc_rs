//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 670/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk670<F: Float>(t12604: F, t943: F, t883: F, t9603: F, t7296: F, t9595: F, t2562: F, t2558: F, t3270: F, t3266: F, t161: F, t165: F, t3234: F, t2685: F, t2684: F, t3209: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12605 = t943 * t12604;
    let t12607 = t883 * t9603;
    let t12608 = t7296 * t12607;
    let t12609 = t943 * t12608;
    let t12611 = t883 * t9595;
    let t12612 = t2562 * t12611;
    let t12613 = t943 * t12612;
    let t12623 = t3270 * t2558;
    let t12624 = t943 * t12623;
    let t12629 = t3266 * t2558;
    let t12630 = t943 * t12629;
    let t12651 = t161 * t165 * t3234;
    let t12652 = t2685 * t12651;
    let t12653 = t2684 * t12652;
    let t12656 = t161 * t165 * t3209;
    (t12605, t12608, t12609, t12612, t12613, t12623, t12624, t12629, t12630, t12651, t12652, t12653, t12656)
}
