//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1030/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1030(t2410: f64, t8787: f64, t9283: f64, t3317: f64, t840: f64, t1120: f64, t4442: f64, t8713: f64, t352: f64, t6126: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9284 = t8787 * t2410;
    let t9285 = t9283 * t9284;
    let t9289 = 7.0_f64 / 144.0_f64 * t840 * t3317;
    let t9290 = t4442 * t1120;
    let t9292 = t8713 * t2410;
    let t9293 = t9283 * t9292;
    let t9296 = t352 * t6126;
    (t9284, t9285, t9289, t9290, t9292, t9293, t9296)
}
