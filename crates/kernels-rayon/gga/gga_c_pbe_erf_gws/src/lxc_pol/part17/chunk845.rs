//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 845/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk845(t2659: f64, t586: f64, t2581: f64, t5312: f64, t2784: f64, t572: f64, t418: f64, t1827: f64, t587: f64, t2816: f64, t636: f64, t197: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7136 = t2659 * t586;
    let t7138 = 16.0_f64 / 45.0_f64 * t7136 * t2581;
    let t7140 = 16.0_f64 / 45.0_f64 * t5312 * t2581;
    let t7141 = t2784 * t572;
    let t7142 = t7141 * t418;
    let t7143 = t1827 * t7142;
    let t7145 = 8.0_f64 / 45.0_f64 * t587 * t7143;
    let t7147 = 8.0_f64 / 45.0_f64 * t2816 * t636;
    let t7148 = t589 * t197;
    (t7136, t7138, t7140, t7145, t7147, t7148)
}
