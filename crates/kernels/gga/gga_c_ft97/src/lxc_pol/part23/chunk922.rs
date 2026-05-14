//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 922/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk922<F: Float>(t28344: F, t3886: F, t14187: F, t684: F, t6921: F, t10007: F, t6849: F, t8392: F, t6940: F, t761: F, t2606: F, t3746: F, t6161: F, t3837: F, t13885: F, t24668: F, t3842: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28345 = t28344 * t3886;
    let t28346 = t14187 * t28345;
    let t28349 = t6921 * t684;
    let t28350 = t10007 * t28349;
    let t28353 = t8392 * t6849;
    let t28355 = t761 * t6940;
    let t28356 = t28355 * t684;
    let t28357 = t2606 * t28356;
    let t28360 = t6161 * t3746;
    let t28361 = t2606 * t28360;
    let t28364 = t6161 * t3837;
    let t28365 = t13885 * t28364;
    let t28368 = t24668 * t3842;
    (t28345, t28346, t28349, t28350, t28353, t28355, t28356, t28357, t28360, t28361, t28364, t28365, t28368)
}
