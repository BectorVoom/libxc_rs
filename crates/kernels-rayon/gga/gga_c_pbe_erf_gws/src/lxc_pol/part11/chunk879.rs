//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 879/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk879(t1113: f64, t15149: f64, t38: f64, t368: f64, t4340: f64, t4348: f64, t4498: f64, t4502: f64, t4505: f64, t4512: f64, t4538: f64, t4541: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15150 = t1113 * t15149;
    let t15651 = t38 * t38;
    let t15652 = 1.0_f64 / t15651;
    let t16191 = t368 * t368;
    let t16192 = 1.0_f64 / t16191;
    let t16329 = 0.12654485932329694421e2_f64 * t4340;
    let t16331 = 0.73024584604562962965e1_f64 * t4348;
    let t16334 = 48.0_f64 * t4498;
    let t16335 = 0.19298189186581325787e3_f64 * t4502;
    let t16336 = 24.0_f64 * t4505;
    let t16337 = 0.38596378373162651572e3_f64 * t4512;
    let t16338 = 4.0_f64 * t4538;
    let t16340 = 24.0_f64 * t4541;
    (t15150, t15651, t15652, t16192, t16329, t16331, t16334, t16335, t16336, t16337, t16338, t16340)
}
