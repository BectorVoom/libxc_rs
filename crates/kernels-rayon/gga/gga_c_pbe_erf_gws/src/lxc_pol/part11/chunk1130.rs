//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1130/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1130(t186: f64, t211: f64, t47948: f64, t47973: f64, t48008: f64, t48037: f64, t650: f64, t32279: f64, t41297: f64, t41300: f64, t47979: f64, t639: f64, t7853: f64) -> (f64, f64, f64, f64, f64) {
    let t48043 = 2.0_f64 / 15.0_f64 * t211 * t186 * t650 * (t47948 + t47973 + t48008 + t48037);
    let t48044 = 16.0_f64 / 45.0_f64 * t32279;
    let t48045 = 64.0_f64 / 45.0_f64 * t41297;
    let t48046 = 32.0_f64 / 45.0_f64 * t41300;
    let t48049 = 64.0_f64 / 27.0_f64 * t639 * t7853 * t47979;
    (t48043, t48044, t48045, t48046, t48049)
}
