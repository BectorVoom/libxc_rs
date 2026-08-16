//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 943/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk943(t313: f64, t39403: f64, t12223: f64, t2464: f64, t2465: f64, t825: f64, t39002: f64, t787: f64, t9824: f64, t13891: f64, t2033: f64, t549: f64) -> (f64, f64, f64, f64, f64) {
    let t47500 = t313 * t39403;
    let t47506 = t825 * t2464 * t2465 * t12223;
    let t47508 = t787 * t39002;
    let t47509 = t47508 * t9824;
    let t47517 = t2033 * t549 * t13891;
    (t47500, t47506, t47508, t47509, t47517)
}
