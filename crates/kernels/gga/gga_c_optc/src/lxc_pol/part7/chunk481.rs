//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 481/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk481<F: Float>(t2427: F, t265: F, t241: F, t6: F, t883: F, t1008: F, t1007: F) -> (F, F, F, F) {
    let t2428 = t2427 * t265;
    let t2430 = 0.19751789702565206229e-1 * t241 * t2428;
    let t2431 = t883 * t6;
    let t2432 = t1008 * t2431;
    let t2433 = t1007 * t2432;
    (t2428, t2430, t2432, t2433)
}
