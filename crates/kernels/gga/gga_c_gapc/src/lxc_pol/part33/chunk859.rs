//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 859/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk859<F: Float>(t19: F, t2207: F, t10346: F, t11210: F, t2580: F, t11214: F, t268: F, t6853: F, t6857: F, t829: F, t3235: F, t3729: F, t125: F, t818: F, t329: F, t2536: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11656 = t2207 * t19;
    let t11657 = t10346 * t11656;
    let t11658 = t11210 * t2580;
    let t11659 = t11657 * t11658;
    let t11661 = t11214 * t268;
    let t11662 = t11661 * t6853;
    let t11663 = t829 * t6857;
    let t11664 = t11662 * t11663;
    let t11666 = t3235 * t3729;
    let t11668 = t125 * t818;
    let t11669 = t11668 * t329;
    let t11670 = t11669 * t2536;
    (t11656, t11657, t11658, t11659, t11661, t11662, t11663, t11664, t11666, t11669, t11670)
}
