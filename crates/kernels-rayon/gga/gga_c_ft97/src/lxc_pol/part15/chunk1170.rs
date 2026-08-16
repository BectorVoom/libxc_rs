//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1170/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1170(t2857: f64, t446: f64, t88153: f64, t41911: f64, t43480: f64, t88252: f64, t89: f64, t10414: f64, t666: f64, t2670: f64, t88239: f64, t19289: f64, t193: f64, t5299: f64) -> (f64, f64, f64, f64, f64) {
    let t89840 = t446 * t2857 * t88153;
    let t89845 = t89 * t41911 * t43480 * t88252;
    let t89851 = t89 * t666 * t10414 * t88252;
    let t89855 = t89 * t666 * t2670 * t88239;
    let t89859 = t89 * t193 * t19289 * t5299;
    (t89840, t89845, t89851, t89855, t89859)
}
