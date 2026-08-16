//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 219/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk219(t422: f64, t626: f64, t625: f64, t11: f64, t624: f64, t203: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t627 = t626 * t422;
    let t628 = t625 * t627;
    let t629 = t11 * t628;
    let t631 = t624 + 0.18891666666666666667e-2_f64 * t629;
    let t632 = t203 * t631;
    let t633 = t632 * t184;
    (t627, t628, t629, t631, t632, t633)
}
