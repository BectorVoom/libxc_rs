//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 529/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk529<F: Float>(t1036: F, t1446: F, t1085: F, t1476: F, t1483: F, t3061: F, t1587: F, t2251: F, t429: F) -> (F, F, F, F) {
    let t4144 = t1446 * t1036;
    let t4182 = t1476 * t1085;
    let t4208 = t1483 * t3061;
    let t4215 = t2251 * t429 * t1587;
    (t4144, t4182, t4208, t4215)
}
