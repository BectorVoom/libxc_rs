//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 332/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk332(t1174: f64, t834: f64, t841: f64, t1167: f64, t334: f64, t218: f64, t219: f64, t1169: f64, t839: f64, t846: f64) -> (f64, f64, f64, f64, f64) {
    let t1175 = t834 * t1174;
    let t1178 = t841 * t1174;
    let t1180 = t334 * t1167;
    let t1182 = t218 * t219 * t1180;
    let t1184 = 0.1898925e1_f64 * t1175 - t839 + 0.8969e0_f64 * t1169 + 0.3071625e0_f64 * t1178 - t846 + 0.24647e0_f64 * t1182;
    (t1175, t1178, t1180, t1182, t1184)
}
