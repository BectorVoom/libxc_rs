//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 835/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk835(t25: f64, t28: f64, t157: f64, t20384: f64, t20394: f64, t182: f64, t11987: f64, t1298: f64, t20216: f64, t20376: f64, t5170: f64, t5397: f64, t12000: f64, t1302: f64, t20385: f64, t20390: f64, t5178: f64, t5966: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t20396 = (t20384 + t20394) * t157;
    let t20398 = 0.19751673498613801407e-1_f64 * t20396 * t182;
    let t20406 = piecewise3(t26, 0.0_f64, 8.0_f64 / 27.0_f64 * t11987 * t20376 - 2.0_f64 / 3.0_f64 * t5170 * t5397 + 2.0_f64 / 3.0_f64 * t1298 * t20216);
    let t20414 = piecewise3(t29, 0.0_f64, 8.0_f64 / 27.0_f64 * t12000 * t20385 - 2.0_f64 / 3.0_f64 * t5178 * t5966 + 2.0_f64 / 3.0_f64 * t1302 * t20390);
    (t20396, t20398, t20406, t20414)
}
