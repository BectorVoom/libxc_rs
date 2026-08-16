//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 862/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk862(t9258: f64, t9259: f64, t2031: f64, t2931: f64, t7700: f64, t2104: f64, t2899: f64, t299: f64, t5591: f64, t5597: f64, t5609: f64, t5614: f64, t5675: f64, t7582: f64, t7585: f64, t7591: f64, t7617: f64, t7621: f64, t7630: f64, t7639: f64, t7694: f64, t9253: f64) -> (f64, f64) {
    let t9260 = t9258 * t9259;
    let t9263 = t2031 * t2931;
    let t9264 = t7700 * t9263;
    let t9267 = -0.95275595817932748827e-4_f64 * t7582 - t7585 + t7591 + t5591 + t5597 / 162.0_f64 + 0.2540682555144873302e-3_f64 * t5609 + t5614 + t7617 + t7621 / 216.0_f64 - t7630 - t7639 - 0.42874018118069736972e-3_f64 * t299 * t9253 + 0.95275595817932748826e-4_f64 * t5675 + 0.25724410870841842183e-2_f64 * t2104 * t9260 - 0.17149607247227894789e-2_f64 * t2899 * t9264 + t7694;
    (t9263, t9267)
}
