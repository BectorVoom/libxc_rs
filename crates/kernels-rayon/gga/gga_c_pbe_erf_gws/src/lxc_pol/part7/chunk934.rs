//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 934/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk934(t1645: f64, t5470: f64, t1627: f64, t5485: f64, t1630: f64, t5484: f64, t639: f64, t5399: f64, t9: f64, t5402: f64, t17037: f64, t219: f64) -> (f64, f64, f64, f64, f64) {
    let t17434 = 8.0_f64 / 9.0_f64 * t5470 * t1645;
    let t17436 = 16.0_f64 / 45.0_f64 * t1627 * t5485;
    let t17438 = t639 * t1630 * t5484;
    let t17439 = 32.0_f64 / 135.0_f64 * t17438;
    let t17440 = t9 * t5399;
    let t17442 = t639 * t17440 * t5402;
    let t17443 = 256.0_f64 / 243.0_f64 * t17442;
    let t17444 = t219 * t17037;
    (t17434, t17436, t17439, t17443, t17444)
}
