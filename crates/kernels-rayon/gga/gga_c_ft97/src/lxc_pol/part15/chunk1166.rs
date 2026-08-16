//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1166/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1166(t4969: f64, t5225: f64, t10248: f64, t446: f64, t1212: f64, t21362: f64, t2665: f64, t666: f64, t792: f64, t86571: f64, t89: f64, t10270: f64, t2345: f64, t88252: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89783 = t4969 * t5225;
    let t89785 = t446 * t10248 * t89783;
    let t89787 = t21362 * t1212;
    let t89789 = t446 * t2665 * t89787;
    let t89794 = t89 * t666 * t792 * t86571;
    let t89798 = t89 * t2345 * t10270 * t88252;
    (t89783, t89785, t89787, t89789, t89794, t89798)
}
