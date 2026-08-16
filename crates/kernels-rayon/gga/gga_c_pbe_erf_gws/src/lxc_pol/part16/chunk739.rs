//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 739/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk739(t441: f64, t4673: f64, t1257: f64, t433: f64, t62: f64, t1260: f64, t70: f64, t4630: f64, t1273: f64, t1276: f64, t155: f64, t174: f64) -> (f64, f64, f64, f64) {
    let t4674 = t4673 * t441;
    let t4678 = 1.0_f64 / t1257 / t433;
    let t4679 = t62 * t4678;
    let t4681 = 1.0_f64 / t1260 / t70;
    let t4682 = t4630 * t4681;
    let t4687 = t174 * t155 * t1273 * t1276;
    (t4674, t4679, t4682, t4687)
}
