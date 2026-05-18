//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1126/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1126<F: Float>(t16377: F, t2030: F, t16390: F, t6799: F, t138: F, t16351: F, t16420: F, t6941: F, t16326: F, t22265: F, t16323: F, t6879: F) -> (F, F, F, F, F, F) {
    let t48388 = t2030 * t16377;
    let t48402 = t6799 * t16390;
    let t48428 = t16351 * t138;
    let t48487 = t6941 * t16420;
    let t48526 = t22265 * t16326;
    let t48528 = t16323 * t6879;
    (t48388, t48402, t48428, t48487, t48526, t48528)
}
