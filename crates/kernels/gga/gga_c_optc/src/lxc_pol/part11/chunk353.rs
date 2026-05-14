//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 353/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk353<F: Float>(t1514: F, t1583: F, t1582: F, t1013: F, t496: F, t1011: F, t429: F) -> (F, F, F, F) {
    let t1584 = t1583 * t1514;
    let t1585 = t1582 * t1584;
    let t1587 = t1013 * t496;
    let t1588 = t1011 * t429 * t1587;
    (t1584, t1585, t1587, t1588)
}
