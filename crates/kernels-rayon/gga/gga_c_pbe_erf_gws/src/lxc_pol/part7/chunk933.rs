//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 933/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk933(t1820: f64, t5014: f64, t5018: f64, t1621: f64, t1724: f64, t1793: f64, t5109: f64, t639: f64, t1620: f64, t5111: f64, t617: f64, t1635: f64, t5470: f64) -> (f64, f64, f64, f64) {
    let t17419 = t1820 * t5018 * t5014;
    let t17420 = 32.0_f64 / 15.0_f64 * t17419;
    let t17425 = 24.0_f64 / 5.0_f64 * t639 * t1621 * t5109 * t1793 * t1724;
    let t17430 = 32.0_f64 / 5.0_f64 * t1620 * t1621 * t5109 * t5111 * t617;
    let t17432 = 8.0_f64 / 15.0_f64 * t5470 * t1635;
    (t17420, t17425, t17430, t17432)
}
