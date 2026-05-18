//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 680/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk680<F: Float>(t12623: F, t943: F, t2558: F, t3266: F, t161: F, t165: F, t3234: F, t2685: F, t2684: F, t3209: F) -> (F, F, F, F, F, F, F) {
    let t12624 = t943 * t12623;
    let t12629 = t3266 * t2558;
    let t12630 = t943 * t12629;
    let t12651 = t161 * t165 * t3234;
    let t12652 = t2685 * t12651;
    let t12653 = t2684 * t12652;
    let t12656 = t161 * t165 * t3209;
    (t12624, t12629, t12630, t12651, t12652, t12653, t12656)
}
