//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 958/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk958(t17689: f64, t17713: f64, t17734: f64, t17765: f64, t185: f64, t186: f64, t598: f64, t4920: f64, t5312: f64, t1403: f64, t1406: f64, t1764: f64, t1820: f64, t1821: f64) -> (f64, f64, f64) {
    let t17771 = 2.0_f64 / 15.0_f64 * t185 * t186 * t598 * (t17689 + t17713 + t17734 + t17765);
    let t17773 = 64.0_f64 / 15.0_f64 * t5312 * t4920;
    let t17778 = 32.0_f64 / 15.0_f64 * t1820 * t1821 * t1406 * t1764 * t1403;
    (t17771, t17773, t17778)
}
