//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 786/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk786(t512: f64, t131: f64, t120: f64, t133: f64, t1365: f64, t5783: f64, t5818: f64, t5787: f64, t1590: f64, t524: f64, t142: f64, t1378: f64, t1971: f64, t5701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5852 = t512 * t512;
    let t5853 = 1.0_f64 / t5852;
    let t5854 = t131 * t5853;
    let t5863 = 0.89405814814814814813e0_f64 * t133 * t1365 * t120;
    let t5864 = t133 * t5783;
    let t5866 = t133 * t5818;
    let t5874 = t133 * t5787;
    let t5887 = t524 * t1590;
    let t5888 = t5887 * t142;
    let t5891 = t5701 * t1378 * t1971;
    (t5854, t5863, t5864, t5866, t5874, t5888, t5891)
}
