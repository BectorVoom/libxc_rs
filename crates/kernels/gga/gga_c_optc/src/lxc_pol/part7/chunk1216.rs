//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1216/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1216<F: Float>(t7372: F, t7375: F, t888: F, t2595: F, t7256: F, t22015: F, t894: F, t2623: F, t7394: F, t2602: F, t530: F, t862: F) -> (F, F, F, F) {
    let t25091 = t7372 * t888 * t7375;
    let t25093 = t2595 * t7256;
    let t25095 = t894 * t25093 * t22015;
    let t25107 = t2623 * t7394;
    let t25112 = t862 * t530 * t2602;
    (t25091, t25095, t25107, t25112)
}
