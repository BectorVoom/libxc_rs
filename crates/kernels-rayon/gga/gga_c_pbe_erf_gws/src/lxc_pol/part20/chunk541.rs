//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 541/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk541(t2741: f64, t619: f64, t1019: f64, t579: f64, t1799: f64, t1033: f64, t636: f64, t1045: f64, t582: f64, t211: f64, t1780: f64, t2676: f64, t2682: f64, t2687: f64, t2691: f64, t2692: f64, t2693: f64, t2694: f64, t2726: f64, t2728: f64, t2732: f64, t2734: f64, t2739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2743 = 4.0_f64 / 15.0_f64 * t2741 * t619;
    let t2745 = 2.0_f64 / 15.0_f64 * t579 * t1019;
    let t2746 = 8.0_f64 / 45.0_f64 * t1799;
    let t2747 = t1033 * t636;
    let t2748 = 4.0_f64 / 45.0_f64 * t2747;
    let t2749 = t582 * t1045;
    let t2750 = t211 * t2749;
    let t2751 = 4.0_f64 / 45.0_f64 * t2750;
    let t2752 = -t2676 + t2682 - t2687 - t2691 - t2692 - t2693 - t1780 + t2694 - t2726 - t2728 + t2732 + t2734 - t2739 + t2743 - t2745 + t2746 + t2748 - t2751;
    (t2743, t2745, t2746, t2747, t2748, t2749, t2750, t2751, t2752)
}
