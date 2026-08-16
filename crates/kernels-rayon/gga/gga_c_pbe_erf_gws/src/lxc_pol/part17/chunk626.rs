//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 626/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk626(t475: f64, t987: f64, t2858: f64, t525: f64, t299: f64, t991: f64, t169: f64, t242: f64, t171: f64, t2522: f64, t1086: f64, t700: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2986 = t475 * t987;
    let t2990 = t525 * t2858;
    let t2994 = t299 * t991;
    let t2996 = t169 * t2994 * t242;
    let t2998 = t171 * t2522;
    let t3003 = t169 * t1086 * t700;
    (t2986, t2990, t2994, t2996, t2998, t3003)
}
