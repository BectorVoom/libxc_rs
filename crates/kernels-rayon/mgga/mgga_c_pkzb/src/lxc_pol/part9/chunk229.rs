//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 229/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk229(t237: f64, t248: f64, t661: f64, t687: f64, t690: f64, t695: f64, t704: f64, t710: f64, t714: f64, t723: f64, t252: f64) -> (f64, f64, f64) {
    let t727 = t237 * (-0.310907e-1_f64 * t690 * t248 + 1.0_f64 * t695 * t704 + t661 - t687 - 0.19751673498613801407e-1_f64 * t710 + 0.5848223622634646207e0_f64 * t714 * t723);
    let t729 = 0.19751673498613801407e-1_f64 * t237 * t710;
    let t730 = t237 * t252;
    (t727, t729, t730)
}
