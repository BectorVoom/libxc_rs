//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 743/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk743<F: Float>(t12608: F, t943: F, t883: F, t9595: F, t2562: F, t2558: F, t3270: F, t3266: F, t161: F, t165: F, t3234: F, t2685: F) -> (F, F, F, F, F, F, F, F, F) {
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
    (t12609, t12612, t12613, t12623, t12624, t12629, t12630, t12651, t12652)
}
