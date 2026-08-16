//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 857/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk857(t1114: f64, t6154: f64, t2362: f64, t2397: f64, t3083: f64, t2366: f64, t3039: f64, t833: f64, t2367: f64, t3047: f64, t1162: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8662 = t1114 * t6154;
    let t8664 = 7.0_f64 / 144.0_f64 * t8662 * t2362;
    let t8666 = 7.0_f64 / 144.0_f64 * t3083 * t2397;
    let t8669 = t3039 * t2366;
    let t8671 = 7.0_f64 / 144.0_f64 * t8669 * t833;
    let t8677 = 7.0_f64 / 144.0_f64 * t2367 * t3047;
    let t8688 = t1162 * t814;
    (t8662, t8664, t8666, t8669, t8671, t8677, t8688)
}
