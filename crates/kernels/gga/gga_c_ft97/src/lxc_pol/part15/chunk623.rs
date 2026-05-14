//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 623/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk623<F: Float>(t5410: F, t8392: F, t1212: F, t2842: F, t5415: F, t312: F, t5225: F, t5393: F, t870: F, t5299: F, t5309: F, t5376: F, t681: F, t89: F, t1775: F, t5346: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19504 = t8392 * t5410;
    let t19506 = t2842 * t1212;
    let t19511 = t8392 * t5415;
    let t19517 = t312 * t5225;
    let t19571 = t870 * t5393;
    let t19576 = t312 * t5299;
    let t19585 = t2842 * t5309;
    let t19635 = t89 * t681 * t5376;
    let t19649 = t1775 * t5346;
    (t19504, t19506, t19511, t19517, t19571, t19576, t19585, t19635, t19649)
}
