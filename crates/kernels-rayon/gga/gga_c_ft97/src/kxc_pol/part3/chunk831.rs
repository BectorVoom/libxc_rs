//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 831/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk831(t16704: f64, t16930: f64, t515: f64, t16150: f64, t3440: f64, t2210: f64, t16169: f64, t3434: f64, t4417: f64, t558: f64, t2221: f64, t609: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16931 = t16704 + t16930;
    let t16932 = t515 * t16931;
    let t16942 = t3440 * t16150;
    let t16943 = t2210 * t16942;
    let t16946 = t3434 * t16169;
    let t16947 = t2210 * t16946;
    let t16950 = t4417 * t558;
    let t16951 = t3434 * t16950;
    let t16952 = t2221 * t16951;
    let t16955 = t4417 * t609;
    (t16932, t16943, t16947, t16950, t16952, t16955)
}
