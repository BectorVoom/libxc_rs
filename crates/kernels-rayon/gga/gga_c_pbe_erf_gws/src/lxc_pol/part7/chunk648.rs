//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 648/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk648(t5058: f64, t5096: f64, t650: f64, t186: f64, t211: f64, t1672: f64, t662: f64, t1794: f64, t582: f64, t648: f64, t213: f64, t1793: f64, t661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5097 = t5058 + t5096;
    let t5098 = t650 * t5097;
    let t5099 = t186 * t5098;
    let t5101 = 2.0_f64 / 15.0_f64 * t211 * t5099;
    let t5102 = t1672 * t662;
    let t5103 = t211 * t5102;
    let t5104 = 4.0_f64 / 45.0_f64 * t5103;
    let t5105 = t582 * t1794;
    let t5106 = t211 * t5105;
    let t5107 = 8.0_f64 / 15.0_f64 * t5106;
    let t5108 = t648 * t648;
    let t5109 = 1.0_f64 / t5108;
    let t5110 = t213 * t5109;
    let t5111 = t1793 * t661;
    (t5097, t5098, t5099, t5101, t5102, t5104, t5105, t5107, t5108, t5109, t5110, t5111)
}
