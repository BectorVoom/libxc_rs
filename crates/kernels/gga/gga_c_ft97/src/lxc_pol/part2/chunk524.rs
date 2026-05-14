//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 524/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk524<F: Float>(t3700: F, t724: F, t446: F, t2999: F, t665: F, t18: F, t669: F, t89: F, t1132: F, t375: F, t1131: F, t668: F, t505: F, t2354: F, t2371: F, t713: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3701 = t724 * t3700;
    let t3702 = t446 * t3701;
    let t3704 = t2999 * t665;
    let t3705 = t669 * t18;
    let t3707 = t89 * t3704 * t3705;
    let t3710 = t89 * t375 * t1132;
    let t3712 = t1131 * t668;
    let t3713 = t3712 * t505;
    let t3714 = t2354 * t3713;
    let t3715 = t446 * t3714;
    let t3717 = t2371 * t1131;
    let t3718 = t3717 * t713;
    (t3701, t3702, t3704, t3705, t3707, t3710, t3712, t3713, t3714, t3715, t3717, t3718)
}
