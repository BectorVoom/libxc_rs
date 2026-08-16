//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 493/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk493(t13996: f64, t739: f64, t3080: f64, t352: f64, t262: f64, t8620: f64, t13862: f64, t335: f64, t3133: f64, t500: f64, t7: f64, t3122: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13997 = t739 * t13996;
    let t13998 = 0.2993560425465952141e-1_f64 * t13997;
    let t14003 = t3080 * t352;
    let t14004 = t262 * t14003;
    let t14005 = t8620 * t14004;
    let t14007 = t13862 * t335;
    let t14008 = t3133 * t14007;
    let t14010 = t7 * t500;
    let t14011 = t3122 * t14010;
    (t13998, t14003, t14004, t14005, t14007, t14008, t14011)
}
