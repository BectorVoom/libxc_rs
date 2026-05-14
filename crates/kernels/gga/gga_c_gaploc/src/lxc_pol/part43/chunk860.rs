//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 860/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk860<F: Float>(t40374: F, t40380: F, t40397: F, t40400: F, t47877: F, t587: F, t912: F, t1: F, t47008: F, t1415: F, t2413: F, t13829: F, t1646: F, t528: F, t13818: F, t1599: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48071 = 0.38342925953920749677e0 * t40374;
    let t48073 = 0.51123901271894332903e0 * t40380;
    let t48074 = 0.38342925953920749677e0 * t40397;
    let t48076 = 0.76685851907841499354e0 * t40400;
    let t48081 = t587 * t912 * t47877;
    let t48086 = t47008 * t1;
    let t48087 = t1415 * t48086;
    let t48088 = t48087 * t2413;
    let t48093 = 0.35750489951850426669e0 * t528 * t13829 * t1646;
    let t48096 = 0.35750489951850426669e0 * t1599 * t13818;
    (t48071, t48073, t48074, t48076, t48081, t48086, t48088, t48093, t48096)
}
