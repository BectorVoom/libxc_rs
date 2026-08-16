//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1131/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1131(t32523: f64, t12538: f64, t2615: f64, t16932: f64, t47391: f64, t5293: f64, t587: f64, t12821: f64, t23123: f64, t5211: f64, t41326: f64, t12783: f64, t2612: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48050 = 32.0_f64 / 45.0_f64 * t32523;
    let t48052 = 32.0_f64 / 9.0_f64 * t2615 * t12538;
    let t48056 = 128.0_f64 / 27.0_f64 * t587 * t5293 * t16932 * t47391;
    let t48059 = 64.0_f64 / 15.0_f64 * t5211 * t23123 * t12821;
    let t48060 = 32.0_f64 / 45.0_f64 * t41326;
    let t48062 = 16.0_f64 / 15.0_f64 * t2612 * t12783;
    (t48050, t48052, t48056, t48059, t48060, t48062)
}
