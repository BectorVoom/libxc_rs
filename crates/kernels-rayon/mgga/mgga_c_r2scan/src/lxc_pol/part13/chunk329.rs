//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 329/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk329(t1053: f64, t1102: f64, t1104: f64, t1068: f64, t49: f64, t415: f64) -> (f64, f64, f64) {
    let t1106 = t1102 * t1053 * t1104;
    let t1108 = 0.15243824895787514157e-3_f64 * t1106 - t1068;
    let t1212 = 1.0_f64 / t49;
    let t1213 = t415 * t415;
    (t1108, t1212, t1213)
}
