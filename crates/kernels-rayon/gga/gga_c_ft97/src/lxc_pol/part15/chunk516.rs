//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 516/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk516(t2345: f64, t5209: f64, t89: f64, t1091: f64, t1212: f64, t2665: f64, t446: f64, t2670: f64, t4917: f64, t666: f64, t4635: f64, t792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5211 = t89 * t2345 * t5209;
    let t5213 = t1091 * t1212;
    let t5214 = t2665 * t5213;
    let t5215 = t446 * t5214;
    let t5217 = t2670 * t4917;
    let t5219 = t89 * t666 * t5217;
    let t5221 = t792 * t4635;
    (t5211, t5213, t5214, t5215, t5217, t5219, t5221)
}
