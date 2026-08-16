//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1489/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1489(t116926: f64, t8260: f64, t2289: f64, t655: f64, t8269: f64, t31027: f64, t31047: f64, t31032: f64, t31055: f64, t31062: f64, t101: f64, t613: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t116927 = t116926 * t8260;
    let t116929 = t2289 * t655;
    let t116930 = t116929 * t8269;
    let t116932 = t31027 * t31047;
    let t116934 = t31032 * t31055;
    let t116936 = t31032 * t31062;
    let t116938 = t613 * t101;
    (t116927, t116929, t116930, t116932, t116934, t116936, t116938)
}
