//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 946/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk946(t17568: f64, t1627: f64, t5481: f64, t1730: f64, t5164: f64, t2730: f64, t16745: f64, t186: f64, t220: f64, t616: f64, t1726: f64, t1750: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17569 = 32.0_f64 / 45.0_f64 * t17568;
    let t17570 = t1627 * t5481;
    let t17571 = 32.0_f64 / 27.0_f64 * t17570;
    let t17573 = 16.0_f64 / 15.0_f64 * t1730 * t5164;
    let t17575 = 16.0_f64 / 15.0_f64 * t2730 * t5164;
    let t17577 = -12.0_f64 * t16745;
    let t17581 = 4.0_f64 / 15.0_f64 * t616 * t186 * t220 * t17577;
    let t17583 = 4.0_f64 / 5.0_f64 * t1750 * t1726;
    (t17569, t17571, t17573, t17575, t17577, t17581, t17583)
}
