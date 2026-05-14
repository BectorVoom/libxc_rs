//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 973/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk973<F: Float>(t2164: F, t7048: F, t2174: F, t7022: F, t7009: F, t7037: F, t146: F, t622: F, t7000: F, t7005: F, t155: F, t6165: F, t693: F, t697: F, t2136: F, t7030: F) -> (F, F, F, F, F, F) {
    let t22994 = t2164 * t7048;
    let t23008 = t7022 * t2174;
    let t23010 = t7037 * t7009;
    let t23013 = t146 * t7000 * t622;
    let t23014 = t23013 * t7005;
    let t23017 = t155 * t693 * t6165;
    let t23018 = t23017 * t697;
    let t23020 = t7030 * t2136;
    (t22994, t23008, t23010, t23014, t23018, t23020)
}
