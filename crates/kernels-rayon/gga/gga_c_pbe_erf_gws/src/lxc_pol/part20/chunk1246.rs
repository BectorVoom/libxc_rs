//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1246/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1246(t53729: f64, t14418: f64, t859: f64, t892: f64, t14135: f64, t3039: f64, t20154: f64, t3067: f64, t4155: f64, t938: f64, t2376: f64, t26617: f64, t810: f64) -> (f64, f64, f64, f64, f64) {
    let t53730 = 7.0_f64 / 1152.0_f64 * t53729;
    let t53761 = t859 * t892 * t14418;
    let t53774 = t3039 * t14135;
    let t53779 = t20154 * t3067 * t4155 * t938;
    let t53784 = t26617 * t2376 * t4155 * t810;
    (t53730, t53761, t53774, t53779, t53784)
}
