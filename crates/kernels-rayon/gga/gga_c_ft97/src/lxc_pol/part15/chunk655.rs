//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 655/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk655(t18132: f64, t3724: f64, t375: f64, t4935: f64, t89: f64, t5054: f64, t4934: f64, t7514: f64, t2336: f64, t4930: f64, t4926: f64, t4918: f64, t9725: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18133 = t3724 * t18132;
    let t18145 = t89 * t375 * t4935;
    let t18148 = t89 * t375 * t5054;
    let t18159 = t7514 * t4934;
    let t18168 = t89 * t2336 * t4930;
    let t18171 = t89 * t2336 * t4926;
    let t18174 = t89 * t9725 * t4918;
    (t18133, t18145, t18148, t18159, t18168, t18171, t18174)
}
