//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1182/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1182(t1193: f64, t2100: f64, t353: f64, t859: f64, t13791: f64, t2387: f64, t2227: f64, t14127: f64, t2397: f64, t1452: f64, t331: f64, t20154: f64, t3067: f64, t4007: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50881 = t859 * t353 * t1193 * t2100;
    let t50884 = t2387 * t13791;
    let t50891 = t859 * t2227;
    let t50904 = t14127 * t2397;
    let t50906 = t1452 * t331;
    let t50919 = t20154 * t3067 * t4007 * t938;
    (t50881, t50884, t50891, t50904, t50906, t50919)
}
