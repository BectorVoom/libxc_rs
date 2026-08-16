//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 92/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk92<F: Float>(t121: F, t423: F, t158: F, t169: F, t172: F, t110: F, t9: F, t19: F, t3: F, t108: F, t14: F, t23: F) -> (F, F, F, F, F, F) {
    let t424 = t423 * t121;
    let t425 = t424 * t158;
    let t426 = t169 * t172;
    let t427 = t9 * t110;
    let t432 = t19 / t3;
    let t433 = t108 * t108;
    let t434 = t433 * t433;
    let t435 = t434 * t108;
    let t436 = t432 * t435;
    let t437 = t23 * t14;
    (t424, t425, t426, t427, t436, t437)
}
