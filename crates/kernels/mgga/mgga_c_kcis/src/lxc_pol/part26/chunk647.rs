//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 647/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk647<F: Float>(t125: F, t7603: F, t86: F, t7577: F, t7584: F, t7587: F, t7593: F, t7595: F, t7598: F, t7601: F, t165: F, t2146: F, t782: F, t826: F, t2153: F, t2533: F) -> (F, F, F, F, F, F) {
    let t7605 = t86 * t125 * t7603;
    let t7607 = -0.69505208333333333333e-3 * t7577 + 0.92754700520833333333e-4 * t7584 + 0.16217881944444444444e-2 * t7587 + 0.69505208333333333333e-3 * t7593 + 0.69505208333333333333e-3 * t7595 - 0.13265555555555555555e-1 * t7598 + 0.99491666666666666664e-2 * t7601 - 0.99491666666666666664e-2 * t7605;
    let t7608 = t7607 * t165;
    let t7609 = t2146 * t782;
    let t7610 = t7609 * t826;
    let t7611 = t2533 * t2153;
    (t7605, t7607, t7608, t7609, t7610, t7611)
}
