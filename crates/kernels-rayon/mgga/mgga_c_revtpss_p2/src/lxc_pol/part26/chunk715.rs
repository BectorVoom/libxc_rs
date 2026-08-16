//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 715/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk715(t128: f64, t121: f64, t22: f64, t2508: f64, t9285: f64, t692: f64, t9288: f64, t124: f64, t624: f64, t138: f64) -> (f64, f64, f64, f64, f64) {
    let t9294 = 1.0_f64/pow_3_2(t128);
    let t9295 = t9294 * t121;
    let t9296 = t9295 * t22;
    let t9298 = t2508 * t9285;
    let t9300 = t692 * t9288;
    let t9302 = t124 * t624;
    let t9303 = t138 * t9302;
    (t9296, t9298, t9300, t9302, t9303)
}
