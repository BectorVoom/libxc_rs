//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 881/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk881(t2148: f64, t9380: f64, t6165: f64, t2294: f64, t3100: f64, t2139: f64, t3115: f64, t2133: f64, t1604: f64, t9377: f64, t3190: f64, t788: f64) -> (f64, f64, f64, f64, f64) {
    let t9381 = t2148 * t9380;
    let t9382 = t6165 * t9381;
    let t9387 = t2294 * t3100;
    let t9388 = t2139 * t9387;
    let t9390 = t2294 * t3115;
    let t9391 = t2133 * t9390;
    let t9397 = t1604 * t9377;
    let t9399 = t788 * t3190;
    (t9382, t9388, t9391, t9397, t9399)
}
