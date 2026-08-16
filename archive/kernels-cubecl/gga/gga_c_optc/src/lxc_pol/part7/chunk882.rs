//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 882/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk882<F: Float>(t123: F, t3126: F, t3108: F, t8469: F, t2856: F, t3119: F, t3118: F, t24: F, t3088: F, t1111: F, t8425: F, t8428: F) -> (F, F, F, F, F, F, F) {
    let t8470 = t123 * t3126;
    let t8471 = t3108 * t8470;
    let t8472 = t8469 * t8471;
    let t8475 = t3119 * t2856;
    let t8476 = t3118 * t8475;
    let t8479 = t24 * t3088;
    let t8480 = t1111 * t8479;
    let t8482 = t8425 * t8428;
    (t8470, t8471, t8472, t8475, t8476, t8480, t8482)
}
