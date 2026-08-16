//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 652/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk652(t4789: f64, t49: f64, t288: f64, t325: f64, t4616: f64, t235: f64, t3807: f64, t511: f64, t2189: f64, t7228: f64, t3350: f64, t201: f64, t4443: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34795 = t4789 * t49;
    let t34796 = t34795 * t288;
    let t34812 = t325 * t4616;
    let t34813 = t235 * t34812;
    let t34828 = t3807 * t511;
    let t34846 = t2189 * t7228;
    let t34847 = t34846 * t3350;
    let t34855 = t201 * t4443;
    (t34795, t34796, t34812, t34813, t34828, t34846, t34847, t34855)
}
