//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 624/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk624<F: Float>(t458: F, t5360: F, t5356: F, t1775: F, t5349: F, t5352: F, t5343: F, t2: F, t5225: F, t10631: F, t5337: F, t19246: F, t19249: F, t19278: F, t19298: F, t19301: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19651 = t458 * t5360;
    let t19653 = t458 * t5356;
    let t19691 = t1775 * t5349;
    let t19693 = t1775 * t5352;
    let t19695 = t1775 * t5343;
    let t19709 = t2 * t5225;
    let t19759 = t10631 * t5337;
    let t19838 = t19246 / 3.0;
    let t19839 = 2.0 / 3.0 * t19249;
    let t19852 = 2.0 / 9.0 * t19278;
    let t19857 = t19298 / 9.0;
    let t19858 = 2.0 / 9.0 * t19301;
    (t19651, t19653, t19691, t19693, t19695, t19709, t19759, t19838, t19839, t19852, t19857, t19858)
}
