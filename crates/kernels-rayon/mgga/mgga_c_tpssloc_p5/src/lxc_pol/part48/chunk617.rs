//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 617/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk617(t1176: f64, t461: f64, t491: f64, t225: f64, t497: f64, t1090: f64, t1186: f64, t2123: f64, t1235: f64, t462: f64, t457: f64, t1240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7284 = t1176 * t461;
    let t7285 = t7284 * t491;
    let t7286 = t225 * t497;
    let t7287 = t7286 * t1090;
    let t7288 = t7285 * t7287;
    let t7291 = t1186 * t2123;
    let t7294 = t1235 * t225;
    let t7295 = t7294 * t497;
    let t7296 = t462 * t7295;
    let t7299 = t457 * t461;
    let t7300 = t7299 * t491;
    let t7301 = t225 * t1240;
    (t7284, t7285, t7286, t7287, t7288, t7291, t7295, t7296, t7299, t7300, t7301)
}
