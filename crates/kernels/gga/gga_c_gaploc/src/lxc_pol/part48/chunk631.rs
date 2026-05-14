//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 631/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk631<F: Float>(t13483: F, t502: F, t11595: F, t948: F, t2508: F, t3650: F, t7301: F, t943: F, t11613: F, t2624: F, t3616: F, t954: F, t2936: F, t3451: F, t13183: F, t11627: F, t7296: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13484 = t502 * t13483;
    let t13486 = t11595 * t948;
    let t13488 = 0.23071578690426672851e-1 * t2508 * t13486;
    let t13489 = t3650 * t7301;
    let t13490 = t943 * t13489;
    let t13492 = t11613 * t2624;
    let t13494 = 0.92286314761706691403e-1 * t2508 * t13492;
    let t13495 = t954 * t3616;
    let t13497 = 0.76905262301422242837e-2 * t2508 * t13495;
    let t13498 = t2936 * t3451;
    let t13501 = 0.1281754371690370714e-2 * t13183;
    let t13503 = t7296 * t883 * t11627;
    (t13484, t13486, t13488, t13489, t13490, t13492, t13494, t13495, t13497, t13498, t13501, t13503)
}
