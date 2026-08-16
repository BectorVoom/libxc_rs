//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1177/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1177(t11505: f64, t494: f64, t97: f64, t2330: f64, t23791: f64, t3446: f64, t37475: f64, t970: f64, t105: f64, t2530: f64, t797: f64, t8296: f64) -> (f64, f64, f64, f64, f64) {
    let t40664 = t97 * t11505 * t494;
    let t40667 = t23791 * t2330;
    let t40672 = t3446 * t37475 * t970;
    let t40681 = t97 * t105 * t2530;
    let t40691 = t797 * t8296;
    (t40664, t40667, t40672, t40681, t40691)
}
