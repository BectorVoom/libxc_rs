//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 769/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk769(t10414: f64, t9571: f64, t2345: f64, t89: f64, t2857: f64, t9583: f64, t446: f64, t2413: f64, t824: f64, t2665: f64, t2739: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10415 = t10414 * t9571;
    let t10417 = t89 * t2345 * t10415;
    let t10419 = t2857 * t9583;
    let t10420 = t446 * t10419;
    let t10422 = t2413 * t824;
    let t10423 = t2665 * t10422;
    let t10424 = t446 * t10423;
    let t10426 = t684 * t2739;
    (t10415, t10417, t10419, t10420, t10422, t10423, t10424, t10426)
}
