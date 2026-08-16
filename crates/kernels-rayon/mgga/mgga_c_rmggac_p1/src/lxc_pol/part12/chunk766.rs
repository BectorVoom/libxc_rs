//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 766/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk766(t2064: f64, t833: f64, t1550: f64, t1338: f64, t2039: f64, t357: f64, t638: f64, t132: f64, t4781: f64, t1343: f64, t2040: f64, t71: f64, t830: f64) -> (f64, f64, f64, f64, f64) {
    let t35765 = t2064 * t833;
    let t35766 = t1550 * t35765;
    let t35772 = t638 * t2039 * t357 * t1338;
    let t35776 = t638 * t2039 * t132 * t4781;
    let t35777 = 0.15243824895787514157e-3_f64 * t35776;
    let t35781 = t638 * t830 * t1343 * t71 * t2040;
    (t35765, t35766, t35772, t35777, t35781)
}
