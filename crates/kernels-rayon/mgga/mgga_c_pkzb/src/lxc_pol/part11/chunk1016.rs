//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1016/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1016(t11184: f64, t6165: f64, t3052: f64, t3747: f64, t11190: f64, t841: f64, t1167: f64, t3730: f64, t218: f64, t219: f64, t11153: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11196 = t6165 * t11184;
    let t11198 = t3052 * t3747;
    let t11200 = t841 * t11190;
    let t11205 = t1167 * t3730;
    let t11207 = t218 * t219 * t11205;
    let t11209 = t334 * t11153;
    (t11196, t11198, t11200, t11205, t11207, t11209)
}
