//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 233/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk233(t374: f64, t930: f64, t423: f64, t920: f64, t420: f64, t419: f64, t417: f64) -> (f64, f64, f64, f64) {
    let t931 = t374 * t930;
    let t934 = t423 * t920;
    let t935 = t420 * t934;
    let t936 = t419 * t935;
    let t938 = t417 + 0.6384360837962962963e-2_f64 * t936;
    (t931, t934, t936, t938)
}
