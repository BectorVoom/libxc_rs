//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 607/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk607(t2568: f64, t5064: f64, t242: f64, t2574: f64, t265: f64, t4934: f64, t1131: f64, t1168: f64, t729: f64, t762: f64, t1091: f64, t1175: f64, t724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5065 = t2568 * t5064;
    let t5066 = t242 * t5065;
    let t5070 = t2574 * t265 * t4934;
    let t5073 = t1131 * t1168;
    let t5075 = t729 * t762 * t5073;
    let t5079 = t724 * t1175 * t1091;
    (t5065, t5066, t5070, t5073, t5075, t5079)
}
