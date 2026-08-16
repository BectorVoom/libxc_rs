//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 814/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk814(t7244: f64, t8437: f64, t36292: f64, t5888: f64, t739: f64, t118: f64, t2001: f64, t2281: f64, t495: f64, t305: f64, t321: f64, t2286: f64, t34881: f64) -> (f64, f64, f64, f64, f64) {
    let t39977 = t7244 * t8437;
    let t39997 = t739 * t36292 * t5888;
    let t40001 = t2001 * t118 * t2281 * t495;
    let t40031 = t2001 * t305 * t2281 * t321;
    let t40045 = t34881 * t2286;
    (t39977, t39997, t40001, t40031, t40045)
}
