//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1094/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1094<F: Float>(t5598: F, t5599: F, t92557: F, t1293: F, t8051: F, t14: F, t22636: F, t70: F, t5612: F, t92433: F, t5611: F, t22609: F, t22632: F, t44991: F, t5522: F, t7837: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t92797 = t5598 * t92557 * t5599;
    let t92809 = t8051 * t1293;
    let t92818 = t22636 * t14;
    let t92819 = t92818 * t70;
    let t92831 = t92433 * t5612;
    let t92832 = t5611 * t92831;
    let t92834 = t92557 * t5612;
    let t92835 = t5611 * t92834;
    let t92837 = t22632 * t22609;
    let t92838 = t5611 * t92837;
    let t92864 = t7837 * t5522 * t44991;
    (t92797, t92809, t92818, t92819, t92831, t92832, t92834, t92835, t92837, t92838, t92864)
}
