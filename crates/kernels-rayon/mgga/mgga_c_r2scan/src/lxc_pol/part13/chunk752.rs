//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 752/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk752(t2127: f64, t6118: f64, t1550: f64, t1569: f64, t2597: f64, t546: f64, t1553: f64, t277: f64, t565: f64, t1582: f64, t259: f64, t503: f64, t6068: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6119 = t6118 * t2127;
    let t6127 = t1569 * t1550;
    let t6132 = t546 * t2597;
    let t6133 = t277 * t1553;
    let t6139 = t565 * t2597;
    let t6148 = t1582 * t259;
    let t6149 = t546 * t6148;
    let t6152 = t565 * t6148;
    let t6155 = t503 * t6068;
    (t6119, t6127, t6132, t6133, t6139, t6148, t6149, t6152, t6155)
}
