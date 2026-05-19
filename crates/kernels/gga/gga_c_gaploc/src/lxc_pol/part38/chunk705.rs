//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 705/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk705<F: Float>(t13382: F, t13418: F, t13456: F, t13481: F, t502: F, t11595: F, t948: F, t2508: F, t3650: F, t7301: F, t943: F, t11613: F, t2624: F) -> (F, F, F, F, F, F, F) {
    let t13483 = t13382 + t13418 + t13456 + t13481;
    let t13484 = t502 * t13483;
    let t13486 = t11595 * t948;
    let t13488 = F::cast_from(0.23071578690426672851e-1_f64) * t2508 * t13486;
    let t13489 = t3650 * t7301;
    let t13490 = t943 * t13489;
    let t13492 = t11613 * t2624;
    (t13483, t13484, t13486, t13488, t13489, t13490, t13492)
}
