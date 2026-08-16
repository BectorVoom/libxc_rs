//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1228/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1228(t1353: f64, t3696: f64, t39284: f64, t39289: f64, t39295: f64, t39298: f64, t39303: f64, t39306: f64, t39314: f64, t39317: f64, t39321: f64, t39326: f64, t39330: f64, t39334: f64, t39338: f64, t39342: f64, t39344: f64) -> f64 {
    let t40726 = t1353 * t3696 + t39284 + t39289 + t39295 + t39298 + t39303 + t39306 - t39314 - t39317 - t39321 + t39326 - t39330 - t39334 + t39338 + t39342 - t39344;
    t40726
}
