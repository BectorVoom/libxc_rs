//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 192/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk192(t137: f64, t131: f64, t120: f64, t133: f64, t156: f64) -> (f64, f64, f64, f64, f64) {
    let t512 = t137 * t137;
    let t513 = 1.0_f64 / t512;
    let t514 = t131 * t513;
    let t517 = 0.28737583333333333333e0_f64 * t133 * t156 * t120;
    let t524 = 1.0_f64 / t131;
    (t512, t513, t514, t517, t524)
}
