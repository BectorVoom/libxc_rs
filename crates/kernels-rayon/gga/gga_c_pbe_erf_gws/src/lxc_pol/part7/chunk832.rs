//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 832/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk832(t108: f64, t182: f64, t267: f64, t1764: f64, t5219: f64, t1660: f64, t597: f64, t1663: f64, t210: f64, t1791: f64, t641: f64, t2659: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7061 = t182 * t108;
    let t7062 = t7061 * t267;
    let t7063 = t5219 * t1764;
    let t7068 = t1660 * t597;
    let t7069 = t7068 * t1663;
    let t7114 = t210 * t108;
    let t7115 = t7114 * t267;
    let t7116 = t641 * t1791;
    let t7136 = t2659 * t586;
    (t7062, t7063, t7068, t7069, t7115, t7116, t7136)
}
