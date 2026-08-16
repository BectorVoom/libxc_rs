//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1037/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1037(t10855: f64, t110: f64, t2591: f64, t481: f64, t560: f64, t11747: f64, t545: f64, t113: f64, t2719: f64, t494: f64, t146: f64, t6533: f64, t978: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25851 = t10855 * t110;
    let t25962 = t2591 * t481;
    let t25968 = t2591 * t560;
    let t25983 = t545 * t11747;
    let t25997 = t2719 * t494 * t113;
    let t26088 = t146 * t6533 * t978;
    (t25851, t25962, t25968, t25983, t25997, t26088)
}
