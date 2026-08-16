//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 545/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk545<F: Float>(t1030: F, t3127: F, t1749: F, t190: F, t653: F) -> (F, F, F) {
    let t3128 = t1030 * t3127;
    let t3129 = t3128 * t1749;
    let t3131 = t653 * t190;
    (t3128, t3129, t3131)
}
