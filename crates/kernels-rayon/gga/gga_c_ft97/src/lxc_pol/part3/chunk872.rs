//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 872/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk872(t17544: f64, t3664: f64, t3640: f64, t3653: f64, t637: f64, t2253: f64, t4865: f64, t4869: f64, t4857: f64, t2281: f64, t4883: f64, t643: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17545 = t17544 * t3664;
    let t17549 = t637 * t3640 * t3653;
    let t17552 = t2253 * t4865;
    let t17554 = t2253 * t4869;
    let t17556 = t2253 * t4857;
    let t17558 = t2281 * t4883;
    let t17560 = t637 * t17558 * t643;
    (t17545, t17549, t17552, t17554, t17556, t17560)
}
