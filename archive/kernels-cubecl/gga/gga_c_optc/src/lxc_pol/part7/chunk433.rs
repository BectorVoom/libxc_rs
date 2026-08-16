//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 433/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk433<F: Float>(t2036: F, t2126: F, t127: F, t2067: F, t5: F, t675: F, t2114: F, t2002: F, t56: F) -> (F, F, F, F) {
    let t2127 = t2126 * t2036;
    let t2131 = t5 * t2067 * t127;
    let t2132 = t675 * t2131;
    let t2135 = t2114 * t127;
    let t2136 = t675 * t2135;
    let t2139 = t2002 * t56;
    (t2127, t2132, t2136, t2139)
}
