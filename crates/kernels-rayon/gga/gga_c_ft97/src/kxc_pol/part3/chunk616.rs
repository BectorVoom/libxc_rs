//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 616/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk616(t1217: f64, t5206: f64, t2660: f64, t4917: f64, t2345: f64, t89: f64, t1091: f64, t1212: f64, t2665: f64, t446: f64, t2670: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5207 = t5206 * t1217;
    let t5209 = t2660 * t4917;
    let t5211 = t89 * t2345 * t5209;
    let t5213 = t1091 * t1212;
    let t5214 = t2665 * t5213;
    let t5215 = t446 * t5214;
    let t5217 = t2670 * t4917;
    let t5219 = t89 * t666 * t5217;
    (t5207, t5209, t5211, t5213, t5214, t5215, t5217, t5219)
}
