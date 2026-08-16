//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 920/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk920(t3793: f64, t870: f64, t2281: f64, t3792: f64, t3102: f64, t3106: f64, t3779: f64, t6290: f64, t6088: f64, t6090: f64, t7955: f64, t8233: f64, t9782: f64, t9797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10009 = t3793 * t870;
    let t10012 = t3792 * t2281;
    let t10013 = t10012 * t870;
    let t10016 = t3106 * t3102;
    let t10019 = t3779 * t6290;
    let t10020 = t10019 * t870;
    let t10027 = -t6088 + 0.23744444444444444444e-1_f64 * t6090 + 0.47488888888888888888e-1_f64 * t7955 - t8233 - 0.17808333333333333333e-1_f64 * t9782 + 0.53425e-1_f64 * t9797;
    (t10009, t10012, t10013, t10016, t10019, t10020, t10027)
}
