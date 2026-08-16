//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3652/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3652<F: Float>(t45000: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F) -> F {
    let t69044 = F::cast_from(0.71233333333333333332e-1_f64) * t68253 + F::cast_from(0.79148148148148148146e-2_f64) * t68255 - F::cast_from(0.52765432098765432097e-2_f64) * t68257 + t45000 - F::cast_from(0.13191358024691358024e-1_f64) * t68262 + F::cast_from(0.19787037037037037037e-1_f64) * t68267 + F::cast_from(0.4274e0_f64) * t68271 + F::cast_from(0.71233333333333333332e-1_f64) * t68275 - F::cast_from(0.23744444444444444444e-1_f64) * t68277 - F::cast_from(0.23744444444444444444e-1_f64) * t68282 - F::cast_from(0.11872222222222222222e-1_f64) * t68287 - F::cast_from(0.71233333333333333332e-1_f64) * t68292;
    t69044
}
