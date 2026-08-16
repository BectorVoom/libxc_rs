//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1097/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1097(t12643: f64, t2612: f64, t10326: f64, t12731: f64, t2790: f64, t3564: f64, t40899: f64, t47565: f64, t47566: f64, t47567: f64, t47568: f64, t47570: f64, t47574: f64, t47576: f64) -> (f64, f64, f64, f64, f64) {
    let t47578 = 128.0_f64 / 81.0_f64 * t2612 * t12643;
    let t47580 = 16.0_f64 / 15.0_f64 * t10326 * t12731;
    let t47582 = 16.0_f64 / 15.0_f64 * t2790 * t12731;
    let t47584 = 16.0_f64 / 5.0_f64 * t40899 * t3564;
    let t47585 = t47565 + t47566 + t47567 - t47568 + t47570 - t47574 + t47576 + t47578 + t47580 + t47582 + t47584;
    (t47578, t47580, t47582, t47584, t47585)
}
