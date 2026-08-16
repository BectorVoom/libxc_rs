//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 506/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk506(t5073: f64, t729: f64, t762: f64, t1091: f64, t1175: f64, t724: f64, t265: f64, t4973: f64, t2594: f64, t4965: f64, t1154: f64, t2475: f64, t91: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5075 = t729 * t762 * t5073;
    let t5079 = t724 * t1175 * t1091;
    let t5083 = t724 * t265 * t4973;
    let t5087 = t2594 * t265 * t4965;
    let t5092 = t1154 * t1154;
    let t5094 = t91 * t2475 * t5092;
    (t5075, t5079, t5083, t5087, t5092, t5094)
}
