//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 303/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk303(t312: f64, t875: f64, t297: f64, t894: f64, t329: f64, t883: f64, t155: f64, t331: f64, t889: f64, t328: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t941 = t312 * t875;
    let t942 = t941 * t297;
    let t943 = t894 * t942;
    let t946 = t329 * t883;
    let t947 = t155 * t946;
    let t951 = 0.50380704458364197288e-2_f64 * t331 * t889;
    let t952 = t155 * t328;
    let t953 = t952 * t892;
    (t942, t943, t946, t947, t951, t952, t953)
}
