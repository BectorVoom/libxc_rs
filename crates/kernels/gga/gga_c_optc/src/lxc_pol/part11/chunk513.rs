//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 513/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk513<F: Float>(t1422: F, t973: F, t1431: F, t993: F, t356: F, t997: F, t996: F) -> (F, F, F, F) {
    let t4009 = t1422 * t973;
    let t4033 = t1431 * t993;
    let t4037 = t997 * t356;
    let t4038 = t996 * t4037;
    (t4009, t4033, t4037, t4038)
}
