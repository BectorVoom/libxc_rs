//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1083/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1083<F: Float>(t1014: F, t23471: F, t287: F, t1010: F, t2253: F, t7314: F, t1006: F, t8378: F, t2317: F, t2325: F, t7230: F, t7234: F) -> (F, F, F, F, F) {
    let t23473 = t23471 * t287 * t1014;
    let t23474 = t1010 * t23473;
    let t23476 = t7314 * t2253;
    let t23481 = t1006 * t8378;
    let t23485 = t2325 * t2317;
    let t23490 = t7230 * t7234;
    (t23474, t23476, t23481, t23485, t23490)
}
