//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 654/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk654(t1806: f64, t579: f64, t1730: f64, t1798: f64, t1734: f64, t582: f64, t616: f64, t596: f64, t188: f64, t1804: f64, t610: f64, t186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5168 = 4.0_f64 / 5.0_f64 * t579 * t1806;
    let t5169 = t1730 * t1798;
    let t5170 = 16.0_f64 / 15.0_f64 * t5169;
    let t5171 = t582 * t1734;
    let t5172 = t616 * t5171;
    let t5173 = 8.0_f64 / 15.0_f64 * t5172;
    let t5174 = t596 * t596;
    let t5175 = 1.0_f64 / t5174;
    let t5176 = t188 * t5175;
    let t5177 = t1804 * t610;
    let t5178 = t5176 * t5177;
    let t5179 = t186 * t5178;
    (t5168, t5170, t5171, t5173, t5174, t5175, t5177, t5178, t5179)
}
