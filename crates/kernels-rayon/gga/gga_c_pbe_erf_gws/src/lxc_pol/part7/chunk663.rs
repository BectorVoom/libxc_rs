//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 663/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk663(t5245: f64, t5272: f64, t598: f64, t186: f64, t185: f64, t1740: f64, t579: f64, t1867: f64, t582: f64, t1660: f64, t9: f64, t1665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5273 = t5245 + t5272;
    let t5274 = t598 * t5273;
    let t5275 = t186 * t5274;
    let t5277 = 2.0_f64 / 15.0_f64 * t185 * t5275;
    let t5278 = t579 * t1740;
    let t5279 = 8.0_f64 / 15.0_f64 * t5278;
    let t5280 = t582 * t1867;
    let t5281 = t185 * t5280;
    let t5282 = 4.0_f64 / 15.0_f64 * t5281;
    let t5283 = t9 * t1660;
    let t5284 = t5283 * t1665;
    (t5273, t5274, t5275, t5277, t5279, t5280, t5282, t5283, t5284)
}
