//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1476/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1476(t1420: f64, t2453: f64, t3908: f64, t1426: f64, t786: f64, t64: f64, t843: f64, t112: f64, t2289: f64, t666: f64, t654: f64, t98: f64, t99: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10165 = t2453 * t1420;
    let t10166 = t10165 * t3908;
    let t10174 = t1420 * t1426;
    let t10175 = t786 * t10174;
    let t10199 = t64 * t843;
    let t10201 = 154.0_f64 / 27.0_f64 * t10199 * t112;
    let t10202 = t2289 * t666;
    let t10207 = t654 * t654;
    let t10208 = 1.0_f64 / t10207;
    let t10226 = t99 * t98;
    (t10165, t10166, t10174, t10175, t10199, t10201, t10202, t10207, t10208, t10226)
}
