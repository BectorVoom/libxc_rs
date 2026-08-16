//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1062/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1062<F: Float>(t6748: F, t9529: F, t6745: F, t539: F, t9521: F, t1245: F, t6322: F, t6326: F, t3386: F, t6617: F, t6636: F, t1294: F, t23017: F) -> (F, F, F, F, F, F, F, F) {
    let t29348 = t9529 * t6748;
    let t29350 = t9529 * t6745;
    let t29352 = t539 * t9521;
    let t29354 = t6322 * t1245;
    let t29356 = t6326 * t1245;
    let t29365 = t3386 * t6617;
    let t29367 = t3386 * t6636;
    let t29441 = t23017 * t1294;
    (t29348, t29350, t29352, t29354, t29356, t29365, t29367, t29441)
}
