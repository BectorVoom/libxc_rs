//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 380/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk380<F: Float>(t2493: F, t3695: F, t2: F, t737: F, t3700: F, t18: F, t738: F, t1152: F, t458: F, t3713: F, t1131: F, t2372: F, t713: F, t192: F, t3821: F, t743: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3914 = t2493 * t3695;
    let t3917 = t737 * t2;
    let t3918 = t3917 * t3700;
    let t3921 = t738 * t18;
    let t3922 = t737 * t3921;
    let t3925 = t458 * t1152;
    let t3927 = t2493 * t3713;
    let t3930 = t2 * t1131;
    let t3932 = t2372 * t3930 * t713;
    let t3936 = t192 * t743 * t3821;
    (t3914, t3917, t3918, t3921, t3922, t3925, t3927, t3930, t3932, t3936)
}
