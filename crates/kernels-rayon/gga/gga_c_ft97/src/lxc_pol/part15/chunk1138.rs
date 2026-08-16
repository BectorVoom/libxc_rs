//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1138/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1138(t734: f64, t88286: f64, t88952: f64, t88983: f64, t89018: f64, t91: f64, t10024: f64, t446: f64, t88612: f64, t724: f64, t88606: f64, t2345: f64, t88252: f64, t89: f64, t9717: f64) -> (f64, f64, f64, f64) {
    let t89022 = t91 * t734 * (t88286 + t88952 + t88983 + t89018);
    let t89027 = t446 * t10024 * t88612;
    let t89030 = t446 * t724 * t88606;
    let t89034 = t89 * t2345 * t9717 * t88252;
    (t89022, t89027, t89030, t89034)
}
