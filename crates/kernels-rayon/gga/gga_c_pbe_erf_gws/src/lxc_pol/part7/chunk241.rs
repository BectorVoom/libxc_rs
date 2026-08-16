//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 241/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk241(t213: f64, t649: f64, t623: f64, t190: f64, t212: f64, t601: f64, t205: f64, t191: f64) -> (f64, f64, f64, f64) {
    let t650 = t213 * t649;
    let t651 = 0.35991666666666666667e-1_f64 * t623;
    let t655 = 0.66666666666666666667e-2_f64 * t190 * t601 * t212;
    let t656 = 1.0_f64 / t205;
    let t657 = t191 * t656;
    (t650, t651, t655, t657)
}
