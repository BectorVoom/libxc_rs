//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2341/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2341<F: Float>(t192: F, t268: F, t9450: F, t9501: F, t2258: F, t2609: F, t706: F, t9476: F, t9508: F, t2582: F, t2584: F, t39480: F) -> (F, F, F, F, F, F) {
    let t39761 = t192 * t268;
    let t39762 = t9450 * t9501;
    let t39764 = F::cast_from(0.1301229756036208781e0_f64) * t39761 * t39762;
    let t39766 = t706 * t2609 * t2258;
    let t39768 = t9476 * t9508;
    let t39770 = F::cast_from(0.19263893255070628431e1_f64) * t39761 * t39768;
    let t39773 = F::cast_from(0.48245938496077605201e2_f64) * t2582 * t39480 * t2584;
    (t39762, t39764, t39766, t39768, t39770, t39773)
}
