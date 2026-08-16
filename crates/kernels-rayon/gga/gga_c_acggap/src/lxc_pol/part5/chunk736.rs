//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 736/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk736(t1268: f64, t1679: f64, t1680: f64, t2831: f64, t495: f64, t694: f64, t1670: f64, t839: f64, t1427: f64, t1674: f64, t695: f64, t467: f64) -> (f64, f64, f64, f64, f64) {
    let t5417 = t1679 * t1680 * t1268;
    let t5419 = t694 * t2831 * t495;
    let t5422 = t694 * t1670 * t839;
    let t5425 = t1674 * t695 * t1427;
    let t5439 = t495 * t467;
    (t5417, t5419, t5422, t5425, t5439)
}
