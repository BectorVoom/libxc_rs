//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 258/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk258<F: Float>(t1006: F, t1007: F, t1002: F, t992: F, t125: F, t594: F, t169: F) -> (F, F, F, F) {
    let t1008 = t1006 * t1007;
    let t1010 = 0.10427789137624512459e-2 * t992 + 0.30368356656884499037e-4 * t1002 - 0.21724560703384400956e-4 * t1008;
    let t1012 = t594 * t125;
    let t1013 = t169 * t1012;
    (t1008, t1010, t1012, t1013)
}
