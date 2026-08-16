//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 203/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk203(t155: f64, t56: f64, t174: f64, t177: f64, t188: f64) -> (f64, f64, f64, f64) {
    let t567 = t155 * t56;
    let t569 = t174 * t567 * t177;
    let t570 = 0.18891666666666666667e-2_f64 * t569;
    let t571 = t56 * t188;
    (t567, t569, t570, t571)
}
