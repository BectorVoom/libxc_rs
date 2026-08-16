//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 962/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk962(t17806: f64, t5516: f64, t583: f64, t184: f64, t202: f64, t5371: f64, t619: f64, t5099: f64, t633: f64, t5029: f64, t639: f64, t7877: f64) -> (f64, f64, f64, f64, f64) {
    let t17807 = 32.0_f64 / 15.0_f64 * t17806;
    let t17808 = t5516 * t583;
    let t17809 = 32.0_f64 / 15.0_f64 * t17808;
    let t17811 = t202 * t5371 * t184;
    let t17813 = 16.0_f64 / 15.0_f64 * t17811 * t619;
    let t17815 = 8.0_f64 / 15.0_f64 * t633 * t5099;
    let t17817 = t639 * t7877 * t5029;
    (t17807, t17809, t17813, t17815, t17817)
}
