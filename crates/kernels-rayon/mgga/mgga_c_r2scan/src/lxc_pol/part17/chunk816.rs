//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 816/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk816(t8590: f64, t86: f64, t2625: f64, t2854: f64, t2858: f64, t3232: f64, t797: f64, t2266: f64, t481: f64, t2333: f64) -> (f64, f64, f64, f64) {
    let t8591 = t8590 * t86;
    let t8592 = 0.19751673498613801407e-1_f64 * t8591;
    let t8595 = t2858 * t2854 * t2625;
    let t8596 = 12.0_f64 * t8595;
    let t8597 = t3232 * t797;
    let t8599 = t2266 * t8597 * t481;
    let t8600 = 3.0_f64 * t8599;
    let t8601 = t3232 * t2333;
    (t8592, t8596, t8600, t8601)
}
