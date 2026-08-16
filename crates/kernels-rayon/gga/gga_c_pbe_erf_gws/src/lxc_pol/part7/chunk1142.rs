//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1142/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1142(t2195: f64, t810: f64, t2407: f64, t858: f64, t6672: f64, t814: f64, t2118: f64, t2189: f64, t875: f64, t824: f64, t343: f64, t6161: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20464 = t2195 * t810;
    let t20466 = t2407 * t858 * t20464;
    let t20468 = t6672 * t20466 / 4.0_f64;
    let t20469 = t2195 * t814;
    let t20470 = t2118 * t20469;
    let t20474 = t875 * t2189;
    let t20475 = t824 * t20474;
    let t20480 = t6161 * t874 * t343;
    (t20468, t20469, t20470, t20474, t20475, t20480)
}
