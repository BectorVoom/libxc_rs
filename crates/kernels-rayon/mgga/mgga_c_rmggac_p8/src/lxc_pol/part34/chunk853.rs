//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 853/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk853(t1587: f64, t262: f64, t3068: f64, t7282: f64, t2039: f64, t2408: f64, t270: f64, t638: f64, t41738: f64, t656: f64, t8941: f64, t2048: f64, t551: f64) -> (f64, f64, f64, f64) {
    let t75277 = t7282 * t3068 * t262 * t1587;
    let t75282 = t638 * t2039 * t2408 * t270;
    let t75285 = t41738 * t656 * t8941;
    let t75298 = t2048 * t551;
    (t75277, t75282, t75285, t75298)
}
