//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2917/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2917<F: Float>(t1432: F, t4107: F, t9288: F, t10107: F, t3964: F, t9285: F, t39494: F, t4096: F, t40270: F, t4089: F, t138: F, t2438: F, t4131: F, t9674: F) -> (F, F, F, F, F) {
    let t47444 = t1432 * t4107 * t9288;
    let t47450 = t3964 * t10107 * t9285;
    let t47454 = F::cast_from(0.20561456923286030469e-1_f64) * t3964 * t4096 * t39494;
    let t47455 = t40270 * t4089;
    let t47466 = t9674 * t138 * t2438 * t4131;
    (t47444, t47450, t47454, t47455, t47466)
}
