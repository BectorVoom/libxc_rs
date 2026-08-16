//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 758/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk758<F: Float>(t1001: F, t7274: F, t999: F, t2367: F, t2562: F, t2360: F, t2368: F, t2341: F, t997: F, t996: F, t2364: F, t2550: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7275 = t7274 * t1001;
    let t7276 = t999 * t7275;
    let t7278 = t2367 * t2562;
    let t7279 = t999 * t7278;
    let t7281 = t2360 * t2368;
    let t7284 = t997 * t2341;
    let t7285 = t996 * t7284;
    let t7288 = t2364 * t2368;
    let t7294 = t2367 * t2550;
    (t7275, t7276, t7278, t7279, t7281, t7284, t7285, t7288, t7294)
}
