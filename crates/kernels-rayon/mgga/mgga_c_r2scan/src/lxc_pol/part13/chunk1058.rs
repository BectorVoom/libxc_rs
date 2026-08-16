//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1058/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1058(t122: f64, t1654: f64, t10673: f64, t10675: f64, t105: f64, t1553: f64, t97: f64, t158: f64, t166: f64, t23: f64, t23102: f64, t261: f64, t603: f64, t784: f64, t875: f64) -> (f64, f64, f64, f64) {
    let t37505 = t1654 * t122;
    let t37507 = t10673 * t10675 * t37505;
    let t37513 = t97 * t105 * t1553;
    let t37523 = t23102 / t23 / t603 * t875 * t158 * t166 * t784 * t261;
    (t37505, t37507, t37513, t37523)
}
