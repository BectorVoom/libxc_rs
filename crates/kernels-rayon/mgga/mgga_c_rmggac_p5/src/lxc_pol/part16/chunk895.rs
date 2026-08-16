//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 895/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk895(t2010: f64, t938: f64, t9719: f64, t1661: f64, t8342: f64, t2415: f64, t5757: f64, t5061: f64, t7487: f64, t9726: f64, t2019: f64, t2020: f64, t9754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44841 = t2010 * t9719 * t938;
    let t44844 = t2010 * t8342 * t1661;
    let t44847 = t2010 * t2415 * t5757;
    let t44850 = t2010 * t2415 * t5061;
    let t44854 = t7487 * t9726;
    let t44857 = t2019 * t2020 * t9754;
    (t44841, t44844, t44847, t44850, t44854, t44857)
}
