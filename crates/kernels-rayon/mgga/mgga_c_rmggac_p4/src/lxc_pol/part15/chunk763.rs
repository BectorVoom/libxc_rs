//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 763/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk763(t1338: f64, t2039: f64, t357: f64, t638: f64, t132: f64, t4781: f64, t1343: f64, t2040: f64, t71: f64, t830: f64, t2046: f64, t2051: f64, t271: f64, t4773: f64) -> (f64, f64, f64, f64) {
    let t35772 = t638 * t2039 * t357 * t1338;
    let t35776 = t638 * t2039 * t132 * t4781;
    let t35777 = 0.15243824895787514157e-3_f64 * t35776;
    let t35781 = t638 * t830 * t1343 * t71 * t2040;
    let t35782 = 0.44715219694310041527e-2_f64 * t35781;
    let t35786 = t2046 * t4773 * t271 * t71 * t2051;
    (t35772, t35777, t35782, t35786)
}
