//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 941/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk941<F: Float>(t2030: F, t6870: F, t2070: F, t6893: F, t2020: F, t6892: F, t2026: F, t2022: F, t6: F, t127: F, t2067: F, t2024: F, t6889: F, t6799: F, t6885: F, t2029: F, t6875: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22238 = t2030 * t6870;
    let t22240 = t6893 * t2070;
    let t22242 = t2020 * t6892;
    let t22243 = t22242 * t2026;
    let t22245 = t2022 * t2022;
    let t22246 = t6 * t22245;
    let t22247 = t22246 * t127;
    let t22251 = t2067 * t2067;
    let t22252 = t6 * t22251;
    let t22253 = t22252 * t2024;
    let t22257 = t22252 * t127;
    let t22261 = t2030 * t6889;
    let t22263 = t6799 * t6885;
    let t22265 = t6875 * t2029;
    (t22238, t22240, t22243, t22245, t22246, t22247, t22251, t22253, t22257, t22261, t22263, t22265)
}
