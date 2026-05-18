//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 508/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk508<F: Float>(t2: F, t2486: F, t3691: F, t2493: F, t3695: F, t737: F, t3700: F, t18: F, t738: F, t1152: F, t458: F, t3713: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3910 = t2486 * t2;
    let t3911 = t3910 * t3691;
    let t3914 = t2493 * t3695;
    let t3917 = t737 * t2;
    let t3918 = t3917 * t3700;
    let t3921 = t738 * t18;
    let t3922 = t737 * t3921;
    let t3925 = t458 * t1152;
    let t3927 = t2493 * t3713;
    (t3910, t3911, t3914, t3917, t3918, t3921, t3922, t3925, t3927)
}
