//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 972/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk972(t10979: f64, t587: f64, t10792: f64, t1821: f64, t10796: f64, t7694: f64, t2768: f64, t950: f64, t1820: f64, t3414: f64, t5129: f64, t2688: f64, t7495: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10981 = 8.0_f64 / 45.0_f64 * t587 * t10979;
    let t10982 = t1821 * t10792;
    let t10984 = 8.0_f64 / 15.0_f64 * t587 * t10982;
    let t10985 = t7694 * t10796;
    let t10987 = 32.0_f64 / 45.0_f64 * t587 * t10985;
    let t10988 = t2768 * t950;
    let t10989 = t7694 * t10988;
    let t10991 = 16.0_f64 / 45.0_f64 * t1820 * t10989;
    let t10992 = t5129 * t3414;
    let t10993 = t587 * t10992;
    let t10994 = 16.0_f64 / 135.0_f64 * t10993;
    let t10995 = t7495 * t2688;
    (t10981, t10984, t10987, t10991, t10994, t10995)
}
