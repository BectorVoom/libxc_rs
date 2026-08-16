//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1096/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1096(t40324: f64, t40327: f64, t40358: f64, t40361: f64, t1037: f64, t41638: f64, t10908: f64, t1820: f64, t1885: f64, t3345: f64, t1010: f64, t40329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47565 = 256.0_f64 / 243.0_f64 * t40324;
    let t47566 = 64.0_f64 / 15.0_f64 * t40327;
    let t47567 = 64.0_f64 / 45.0_f64 * t40358;
    let t47568 = 32.0_f64 / 15.0_f64 * t40361;
    let t47570 = 16.0_f64 / 45.0_f64 * t41638 * t1037;
    let t47574 = 8.0_f64 / 5.0_f64 * t1820 * t1885 * t10908 * t3345;
    let t47576 = 32.0_f64 / 15.0_f64 * t40329 * t1010;
    (t47565, t47566, t47567, t47568, t47570, t47574, t47576)
}
