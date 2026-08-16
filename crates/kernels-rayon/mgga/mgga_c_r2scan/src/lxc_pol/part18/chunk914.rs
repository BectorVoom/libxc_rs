//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 914/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk914(t9793: f64, t9794: f64, t9798: f64, t9799: f64, t9802: f64, t9810: f64, t9818: f64, t9829: f64, t1569: f64, t3052: f64, t2987: f64, t352: f64) -> (f64, f64, f64) {
    let t9832 = t9793 + t9794 + t9798 + t9799 + t9802 + t9810 + t9818 + t9829;
    let t10024 = t1569 * t3052;
    let t10533 = t352 * t2987;
    (t9832, t10024, t10533)
}
