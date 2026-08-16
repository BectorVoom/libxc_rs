//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1182/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1182(t2227: f64, t859: f64, t14127: f64, t2397: f64, t1452: f64, t331: f64, t13784: f64, t13808: f64, t2271: f64, t332: f64, t822: f64, t824: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50891 = t859 * t2227;
    let t50904 = t14127 * t2397;
    let t50906 = t1452 * t331;
    let t50927 = t13808 * t13784;
    let t50935 = t2271 * t332;
    let t50936 = t822 * t50935;
    let t50942 = t824 * t838;
    (t50891, t50904, t50906, t50927, t50935, t50936, t50942)
}
