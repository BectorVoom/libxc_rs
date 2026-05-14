//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 620/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk620<F: Float>(t1882: F, t2811: F, t2807: F, t295: F, t9568: F, t2803: F, t8232: F, t842: F, t10397: F, t2846: F, t2899: F, t5: F, t2253: F, t2953: F, t170: F, t328: F, t8715: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10750 = t1882 * t2811;
    let t10752 = t1882 * t2807;
    let t10758 = t9568 * t295;
    let t10771 = t1882 * t2803;
    let t10773 = t8232 * t842;
    let t10797 = 28.0 / 27.0 * t10397;
    let t10804 = t1882 * t2846;
    let t10829 = t5 * t2899;
    let t10835 = t2253 * t2953;
    let t10838 = 20.0 / 27.0 * t170 * t8715 * t328;
    (t10750, t10752, t10758, t10771, t10773, t10797, t10804, t10829, t10835, t10838)
}
