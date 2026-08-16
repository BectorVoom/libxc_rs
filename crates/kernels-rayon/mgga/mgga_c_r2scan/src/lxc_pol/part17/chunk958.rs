//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 958/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk958(t3574: f64, t481: f64, t3582: f64, t106: f64, t2530: f64, t97: f64, t2847: f64, t797: f64, t2526: f64, t2333: f64, t983: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11509 = t3574 * t481;
    let t11518 = t3582 * t481;
    let t11523 = t97 * t106 * t2530;
    let t11531 = t797 * t2847;
    let t11550 = t797 * t2526;
    let t11554 = t2333 * t983;
    let t11555 = t11554 * t795;
    (t11509, t11518, t11523, t11531, t11550, t11554, t11555)
}
