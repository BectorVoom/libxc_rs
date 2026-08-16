//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 925/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk925(t5631: f64, t759: f64, t285: f64, t4576: f64, t762: f64, t413: f64, t5772: f64, t5773: f64, t5832: f64, t5833: f64, t1: f64, t119: f64, t6045: f64) -> (f64, f64, f64, f64, f64) {
    let t19182 = 0.78054266140918933351e0_f64 * t5631 * t759;
    let t19203 = 0.11622696607154767747e-2_f64 * t762 * t4576 * t285;
    let t19229 = 0.15589466666666666666e2_f64 * t5772 * t5773 * t413;
    let t19232 = 0.26116266666666666667e1_f64 * t5832 * t5833 * t413;
    let t19247 = t6045 * t1 * t119;
    (t19182, t19203, t19229, t19232, t19247)
}
