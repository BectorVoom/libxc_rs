//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2758/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2758(t192: f64, t268: f64, t9450: f64, t9501: f64, t2258: f64, t2609: f64, t706: f64, t9476: f64, t9508: f64, t2582: f64, t2584: f64, t39480: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39761 = t192 * t268;
    let t39762 = t9450 * t9501;
    let t39764 = 0.1301229756036208781e0_f64 * t39761 * t39762;
    let t39766 = t706 * t2609 * t2258;
    let t39768 = t9476 * t9508;
    let t39770 = 0.19263893255070628431e1_f64 * t39761 * t39768;
    let t39773 = 0.48245938496077605201e2_f64 * t2582 * t39480 * t2584;
    (t39762, t39764, t39766, t39768, t39770, t39773)
}
