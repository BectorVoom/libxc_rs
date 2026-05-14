//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 829/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk829<F: Float>(t3120: F, t8451: F, t2860: F, t3119: F, t3118: F, t22: F, t3145: F, t1122: F, t2850: F, t3104: F, t3126: F, t4357: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8452 = t8451 * t3120;
    let t8455 = t3119 * t2860;
    let t8456 = t3118 * t8455;
    let t8459 = t22 * t3145;
    let t8460 = t8459 * t1122;
    let t8461 = t3119 * t2850;
    let t8462 = t8460 * t8461;
    let t8465 = t3104 * t3126;
    let t8466 = t8465 * t4357;
    let t8469 = t3104 * t1122;
    (t8452, t8455, t8456, t8459, t8460, t8461, t8462, t8465, t8466, t8469)
}
