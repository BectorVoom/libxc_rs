//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 538/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk538(t511: f64, t899: f64, t27: f64, t649: f64, t794: f64, t2084: f64, t321: f64, t2134: f64, t1343: f64, t265: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7282 = t899 * t511;
    let t7284 = t27 * t649 * t794;
    let t7285 = t7282 * t7284;
    let t7286 = 0.20455996240684006296e-1_f64 * t7285;
    let t7287 = t2084 * t321;
    let t7288 = t27 * t7287;
    let t7289 = t2134 * t7288;
    let t7292 = t265 * t1343 * t71;
    (t7282, t7284, t7286, t7288, t7289, t7292)
}
