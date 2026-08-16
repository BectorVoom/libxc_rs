//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 721/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk721(t3450: f64, t582: f64, t561: f64, t3414: f64, t5129: f64, t587: f64, t3454: f64, t572: f64, t3402: f64, t4934: f64, t1620: f64, t3406: f64, t5137: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10972 = t582 * t3450;
    let t10973 = t561 * t10972;
    let t10992 = t5129 * t3414;
    let t10993 = t587 * t10992;
    let t11005 = t3454 * t572;
    let t11019 = t4934 * t3402;
    let t11020 = t1620 * t11019;
    let t11022 = t5137 * t3406;
    (t10972, t10973, t10992, t10993, t11005, t11019, t11020, t11022)
}
