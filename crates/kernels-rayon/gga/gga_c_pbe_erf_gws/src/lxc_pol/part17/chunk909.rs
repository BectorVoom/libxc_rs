//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 909/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk909(t1885: f64, t7936: f64, t587: f64, t589: f64, t837: f64, t2621: f64, t5557: f64, t1023: f64, t1672: f64, t616: f64, t2786: f64, t579: f64) -> (f64, f64, f64, f64, f64) {
    let t7937 = t1885 * t7936;
    let t7939 = 4.0_f64 / 5.0_f64 * t587 * t7937;
    let t7940 = t837 * t589;
    let t7941 = t7940 * t2621;
    let t7942 = t587 * t7941;
    let t7943 = 8.0_f64 / 27.0_f64 * t7942;
    let t7944 = 16.0_f64 / 135.0_f64 * t5557;
    let t7945 = t1672 * t1023;
    let t7946 = t616 * t7945;
    let t7947 = 8.0_f64 / 135.0_f64 * t7946;
    let t7949 = 4.0_f64 / 15.0_f64 * t579 * t2786;
    (t7939, t7943, t7944, t7947, t7949)
}
