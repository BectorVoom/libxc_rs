//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1020/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1020(t8292: f64, t8308: f64, t405: f64, t921: f64, t758: f64, t2099: f64, t3201: f64, t918: f64, t178: f64, t3212: f64, t915: f64) -> (f64, f64, f64, f64, f64) {
    let t8309 = t8292 + t8308;
    let t8311 = t405 * t8309 * t921;
    let t8312 = t758 * t8311;
    let t8315 = t2099 * t3201;
    let t8317 = 0.28582678745379824648e-3_f64 * t918 * t8315;
    let t8318 = t3212 * t178;
    let t8319 = t915 * t8318;
    (t8309, t8311, t8312, t8317, t8319)
}
