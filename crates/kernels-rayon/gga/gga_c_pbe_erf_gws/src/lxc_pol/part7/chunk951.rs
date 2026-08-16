//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 951/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk951(t1765: f64, t1827: f64, t1866: f64, t587: f64, t4958: f64, t5543: f64, t610: f64, t1620: f64, t4902: f64, t4934: f64, t1416: f64, t4927: f64, t4928: f64, t639: f64) -> (f64, f64, f64, f64) {
    let t17634 = 16.0_f64 / 15.0_f64 * t587 * t1827 * t1765 * t1866;
    let t17638 = 32.0_f64 / 9.0_f64 * t587 * t5543 * t4958 * t610;
    let t17640 = t1620 * t4934 * t4902;
    let t17641 = 64.0_f64 / 45.0_f64 * t17640;
    let t17645 = 16.0_f64 / 15.0_f64 * t639 * t4927 * t4928 * t1416;
    (t17634, t17638, t17641, t17645)
}
