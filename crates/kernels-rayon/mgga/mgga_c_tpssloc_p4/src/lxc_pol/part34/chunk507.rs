//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 507/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk507(t1752: f64, t225: f64, t1243: f64, t5000: f64, t1390: f64, t1845: f64, t193: f64, t531: f64, t1799: f64, t571: f64, t1408: f64, t3664: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5055 = t1752 * t225;
    let t5064 = t5000 * t1243;
    let t5122 = t1845 * t1390;
    let t5126 = t193 * t531;
    let t5127 = t571 * t1799;
    let t5134 = t3664 * t1408;
    (t5055, t5064, t5122, t5126, t5127, t5134)
}
