//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 788/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk788<F: Float>(t9263: F, t9278: F, t993: F, t20671: F, t31041: F, t34818: F, t34264: F, t7030: F, t10177: F, t10523: F, t544: F, t899: F, t913: F) -> (F, F, F, F) {
    let t41686 = t9263 * t993 * t9278;
    let t41689 = t31041 * t20671 * t34818;
    let t41691 = t34264 * t7030;
    let t41696 = t544 * t10523 * t899 * t913 * t10177;
    (t41686, t41689, t41691, t41696)
}
