//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3871/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3871(t221: f64, t22274: f64, t3978: f64, t46716: f64, t22279: f64, t9921: f64, t22255: f64, t3930: f64, t22259: f64, t9976: f64, t22125: f64, t2713: f64, t3964: f64) -> (f64, f64, f64, f64, f64) {
    let t74419 = t221 * t22274;
    let t74421 = t3978 * t46716 * t74419;
    let t74423 = t221 * t22279;
    let t74425 = t3978 * t9921 * t74423;
    let t74427 = t3930 * t22255;
    let t74429 = t9976 * t22259;
    let t74437 = t3964 * t2713 * t22125;
    (t74421, t74425, t74427, t74429, t74437)
}
