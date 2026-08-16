//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 56/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk56(t15: f64, t12: f64, t56: f64, t58: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t148 = f64::sqrt(4.0_f64);
    let t149 = t148 * t15;
    let t151 = 0.3138525e-1_f64 * t12;
    let t152 = 1.0_f64 + 0.22225e-1_f64 * t149 + t151;
    let t153 = t152 * t152;
    let t154 = 1.0_f64 / t153;
    let t158 = 1.0_f64 - 0.2363e1_f64 * t58 * t56 * t60;
    (t148, t149, t151, t152, t153, t154, t158)
}
