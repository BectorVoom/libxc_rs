//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 565/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk565<F: Float>(t3712: F, t505: F, t2354: F, t446: F, t1131: F, t2371: F, t713: F, t193: F, t89: F, t2382: F, t688: F, t2379: F) -> (F, F, F, F, F, F, F) {
    let t3713 = t3712 * t505;
    let t3714 = t2354 * t3713;
    let t3715 = t446 * t3714;
    let t3717 = t2371 * t1131;
    let t3718 = t3717 * t713;
    let t3720 = t89 * t193 * t3718;
    let t3722 = t688 * t2382;
    let t3723 = t2379 * t3722;
    (t3713, t3714, t3715, t3717, t3718, t3720, t3723)
}
