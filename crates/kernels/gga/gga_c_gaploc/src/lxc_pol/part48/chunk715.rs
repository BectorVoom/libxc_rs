//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 715/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk715<F: Float>(t33360: F, t787: F, t9824: F, t33348: F, t10892: F, t2021: F, t7372: F, t13141: F, t2464: F, t2684: F, t2617: F, t3451: F, t7803: F, t13069: F, t7416: F, t10040: F, t25198: F) -> (F, F, F, F, F, F, F) {
    let t43522 = t787 * t33360 * t9824;
    let t43526 = t787 * t33348 * t9824;
    let t43529 = t2021 * t10892 * t7372;
    let t43581 = t2684 * t2464 * t13141;
    let t43609 = t7803 * t3451 * t2617;
    let t43611 = t7416 * t13069;
    let t43646 = t25198 * t10040;
    (t43522, t43526, t43529, t43581, t43609, t43611, t43646)
}
