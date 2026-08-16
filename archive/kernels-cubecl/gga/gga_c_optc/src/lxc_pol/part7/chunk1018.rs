//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1018/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1018<F: Float>(t141: F, t22052: F, t659: F, t661: F, t6923: F, t2030: F, t6870: F, t2070: F, t6893: F, t2020: F, t6892: F, t2026: F) -> (F, F, F, F, F) {
    let t22233 = t659 * t141 * t22052;
    let t22236 = t6923 * t661;
    let t22238 = t2030 * t6870;
    let t22240 = t6893 * t2070;
    let t22242 = t2020 * t6892;
    let t22243 = t22242 * t2026;
    (t22233, t22236, t22238, t22240, t22243)
}
