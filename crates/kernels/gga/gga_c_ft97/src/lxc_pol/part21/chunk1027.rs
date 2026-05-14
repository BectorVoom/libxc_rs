//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1027/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1027<F: Float>(t22742: F, t397: F, t5598: F, t5599: F, t92557: F, t1293: F, t8051: F, t5612: F, t5611: F, t44991: F, t5522: F, t7837: F, t22759: F, t444: F, t3076: F, t1609: F, t47: F) -> (F, F, F, F, F, F, F, F) {
    let t92786 = t22742 * t397;
    let t92797 = t5598 * t92557 * t5599;
    let t92809 = t8051 * t1293;
    let t92834 = t92557 * t5612;
    let t92835 = t5611 * t92834;
    let t92864 = t7837 * t5522 * t44991;
    let t92872 = t22759 * t444;
    let t92873 = t3076 * t92872;
    let t92895 = t1609 * t47;
    (t92786, t92797, t92809, t92834, t92835, t92864, t92873, t92895)
}
