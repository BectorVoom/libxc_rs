//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1291/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1291(t11233: f64, t18589: f64, t18592: f64, t851: f64, t11286: f64, t2281: f64, t10006: f64, t10016: f64, t10019: f64, t10020: f64, t11167: f64, t11269: f64, t11287: f64, t18706: f64, t2257: f64, t22767: f64, t2279: f64, t22829: f64, t3102: f64, t31394: f64, t31397: f64, t31400: f64, t31404: f64, t31407: f64, t3796: f64, t6288: f64, t6313: f64, t8120: f64, t870: f64) -> (f64, f64) {
    let t31411 = 0.24955700379505800916e5_f64 * t18589 * t11233 * t18592 * t851;
    let t31430 = t11286 * t2281;
    let t31437 = t31394 + t31397 + t31400 - t31404 - t31407 - t31411 - 0.57895126195293126241e3_f64 * t22829 * t10006 + 0.1929837539843104208e3_f64 * t8120 * t10016 + 0.62071215503128080361e4_f64 * t22767 * t10020 + 0.11579025239058625248e4_f64 * t6288 * t11269 * t870 - 0.57895126195293126243e3_f64 * t6313 * t3796 * t3102 - 0.24828486201251232145e5_f64 * t18706 * t11167 * t870 - 2.0_f64 * t2257 * t11287 * t870 + 0.32163958997385070134e2_f64 * t2279 * t31430 * t870 + 0.6207121550312808036e4_f64 * t6288 * t10019 * t3102;
    (t31411, t31437)
}
