//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1286/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1286(t51351: f64, t9509: f64, t14101: f64, t8842: f64, t4028: f64, t8856: f64, t14046: f64, t3184: f64, t3148: f64, t3123: f64, t51309: f64, t14015: f64, t9467: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54310 = t51351 * t9509;
    let t54315 = t14101 * t8842;
    let t54317 = t4028 * t8856;
    let t54319 = t14046 * t3184;
    let t54322 = t14046 * t3148;
    let t54324 = t3123 * t51309;
    let t54326 = t14015 * t9467;
    (t54310, t54315, t54317, t54319, t54322, t54324, t54326)
}
