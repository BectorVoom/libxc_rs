//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1227/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1227<F: Float>(t116: F, t27071: F, t428: F, t1111: F, t1115: F, t1781: F, t24: F, t8483: F, t2849: F, t371: F, t26336: F, t22035: F, t1114: F, t22046: F, t3097: F, t530: F) -> (F, F, F, F, F, F, F) {
    let t27074 = 5.0 / 486.0 * t428 * t116 * t27071;
    let t27076 = t1111 * t1781 * t1115;
    let t27079 = t1111 * t24 * t8483;
    let t27082 = 1.0 / t371 / t2849;
    let t27083 = t27082 * t26336;
    let t27084 = t27083 * t22035;
    let t27088 = t1114 * t22046;
    let t27093 = t1111 * t530 * t3097;
    (t27074, t27076, t27079, t27082, t27084, t27088, t27093)
}
