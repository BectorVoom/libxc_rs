//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 61/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk61<F: Float>(t103: F, t160: F, t161: F, t164: F, t99: F, t115: F) -> (F, F) {
    let t168 = F::cast_from(0.619125e-2_f64) * t160 * t161 - F::cast_from(0.79593333333333333331e-1_f64) * t103 * t164 * t99;
    let t169 = t168 * t115;
    (t168, t169)
}
