//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 584/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk584(t4805: f64, t605: f64, t144: f64, t1053: f64, t3578: f64, t1017: f64, t1060: f64, t574: f64, t167: f64, t4714: f64, t920: f64, t2222: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4806 = t605 * t4805;
    let t4807 = t144 * t4806;
    let t4810 = t3578 * t1053;
    let t4811 = t144 * t4810;
    let t4815 = t574 * t1060 * t1017;
    let t4819 = t574 * t167 * t4714;
    let t4822 = t920 * t1017;
    let t4823 = t2222 * t4822;
    (t4806, t4807, t4810, t4811, t4815, t4819, t4822, t4823)
}
