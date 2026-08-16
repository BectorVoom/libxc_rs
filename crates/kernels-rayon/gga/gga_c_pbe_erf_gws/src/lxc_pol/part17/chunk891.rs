//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 891/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk891(t256: f64, t7733: f64, t1918: f64, t2654: f64, t5384: f64, t5387: f64, t5388: f64, t7689: f64, t7693: f64, t7697: f64, t7702: f64, t7708: f64, t7710: f64, t7712: f64, t7715: f64, t7719: f64, t7724: f64, t7728: f64, t7732: f64) -> f64 {
    let t7734 = t7733 * t256;
    let t7736 = t2654 * t1918;
    let t7738 = t7689 + t7693 - t7697 + t7702 + t7708 + t7710 - t7712 - t7715 + t7719 - t7724 - t5384 + t5387 + 2.0_f64 / 9.0_f64 * t5388 + t7728 + t7732 + t7734 / 3.0_f64 + 0.12155555555555555555e0_f64 * t7736;
    t7738
}
