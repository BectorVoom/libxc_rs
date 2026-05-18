//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 579/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk579<F: Float>(t3826: F, t661: F, t1140: F, t1882: F, t1131: F, t713: F, t2574: F, t265: F, t766: F, t729: F, t762: F, t1091: F, t724: F, t773: F) -> (F, F, F, F, F, F, F) {
    let t3827 = t661 * t3826;
    let t3835 = t1882 * t1140;
    let t3837 = t1131 * t713;
    let t3839 = t2574 * t265 * t3837;
    let t3842 = t1131 * t766;
    let t3844 = t729 * t762 * t3842;
    let t3848 = t724 * t773 * t1091;
    (t3827, t3835, t3837, t3839, t3842, t3844, t3848)
}
