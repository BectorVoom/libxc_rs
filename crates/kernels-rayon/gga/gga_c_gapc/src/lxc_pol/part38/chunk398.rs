//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 398/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk398(t1936: f64, t581: f64, t144: f64, t481: f64, t152: f64, t583: f64, t6: f64, t1524: f64, t188: f64, t178: f64, t1: f64, t172: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1937 = t581 * t1936;
    let t1938 = t481 * t144;
    let t1939 = t1938 * t152;
    let t1940 = t583 * t6;
    let t1941 = t1939 * t1940;
    let t1944 = t1524 * t188;
    let t1945 = t178 * t1944;
    let t1946 = t172 * t1;
    (t1937, t1938, t1939, t1941, t1944, t1945, t1946)
}
