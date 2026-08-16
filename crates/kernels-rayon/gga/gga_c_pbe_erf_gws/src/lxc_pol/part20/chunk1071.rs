//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1071/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1071(t12039: f64, t326: f64, t9385: f64, t6252: f64, t3037: f64, t5: f64, t337: f64, t2121: f64, t1076: f64, t814: f64, t2255: f64, t3258: f64) -> (f64, f64, f64, f64, f64) {
    let t12040 = 7.0_f64 / 288.0_f64 * t12039;
    let t12041 = t326 * t9385;
    let t12042 = t12041 * t6252;
    let t12043 = t5 * t3037;
    let t12044 = t337 * t12043;
    let t12045 = t2121 * t12044;
    let t12047 = t12042 * t12045 / 48.0_f64;
    let t12048 = t1076 * t814;
    let t12050 = t2255 * t3258 * t12048;
    (t12040, t12041, t12044, t12047, t12050)
}
