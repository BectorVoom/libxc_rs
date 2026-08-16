//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1116/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1116(t2665: f64, t43409: f64, t446: f64, t10388: f64, t505: f64, t668: f64, t10419: f64, t1882: f64, t2405: f64, t2739: f64, t10409: f64, t10423: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43411 = t446 * t2665 * t43409;
    let t43414 = t10388 * t668 * t505;
    let t43416 = t446 * t2665 * t43414;
    let t43418 = t1882 * t10419;
    let t43420 = t2405 * t2739;
    let t43422 = t446 * t10409 * t43420;
    let t43424 = t1882 * t10423;
    (t43411, t43414, t43416, t43418, t43420, t43422, t43424)
}
