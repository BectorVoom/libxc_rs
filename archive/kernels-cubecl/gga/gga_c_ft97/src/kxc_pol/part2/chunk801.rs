//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 801/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk801<F: Float>(t12345: F, t12595: F, t515: F, t1053: F, t2157: F, t2179: F, t3565: F, t609: F, t2180: F, t9439: F, t3478: F, t379: F) -> (F, F, F, F, F) {
    let t12596 = t12345 + t12595;
    let t12597 = t515 * t12596;
    let t12599 = t1053 * t2157;
    let t12600 = t2179 * t12599;
    let t12602 = t3565 * t609;
    let t12603 = t2179 * t12602;
    let t12605 = t1053 * t2180;
    let t12606 = t9439 * t12605;
    let t12609 = t3478 * t379;
    (t12597, t12600, t12603, t12606, t12609)
}
