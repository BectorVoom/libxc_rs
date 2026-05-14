//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 998/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk998<F: Float>(t19: F, t40355: F, t4961: F, t7481: F, t7433: F, t123: F, t769: F, t4971: F, t7878: F, t893: F, t4979: F, t4937: F, t530: F, t862: F, t2372: F, t4895: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40356 = t40355 * t19;
    let t40391 = t7481 * t4961;
    let t40480 = t7433 * t4961;
    let t40489 = t123 * t769;
    let t40526 = t7878 * t4971;
    let t40527 = t893 * t40526;
    let t40538 = t7878 * t4979;
    let t40539 = t893 * t40538;
    let t40677 = t862 * t530 * t4937;
    let t40764 = t4895 * t2372;
    (t40356, t40391, t40480, t40489, t40526, t40527, t40538, t40539, t40677, t40764)
}
