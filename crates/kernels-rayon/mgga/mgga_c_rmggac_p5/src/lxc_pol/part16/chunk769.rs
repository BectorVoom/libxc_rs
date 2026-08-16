//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 769/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk769(t36172: f64, t661: f64, t35875: f64, t851: f64, t35924: f64, t854: f64, t305: f64, t3899: f64, t655: f64, t2067: f64, t25525: f64, t2078: f64, t3839: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36173 = t661 * t36172;
    let t36188 = t851 * t35875;
    let t36190 = t854 * t35924;
    let t36200 = t305 * t3899;
    let t36204 = t655 * t36172;
    let t36250 = t25525 * t2067;
    let t36254 = t3839 * t2078;
    (t36173, t36188, t36190, t36200, t36204, t36250, t36254)
}
