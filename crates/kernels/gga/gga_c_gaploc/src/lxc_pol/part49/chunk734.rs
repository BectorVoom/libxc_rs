//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 734/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk734<F: Float>(t747: F, t9765: F, t1959: F, t3259: F, t203: F, t9127: F, t3116: F, t475: F) -> (F, F, F, F) {
    let t29646 = t9765 * t747;
    let t29650 = t3259 * t1959;
    let t29661 = t203 * t9127;
    let t29853 = t3116 * t475;
    (t29646, t29650, t29661, t29853)
}
