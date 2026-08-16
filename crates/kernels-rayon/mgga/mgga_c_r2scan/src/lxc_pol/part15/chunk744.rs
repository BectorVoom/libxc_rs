//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 744/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk744(t166: f64, t6044: f64, t759: f64, t122: f64, t1415: f64, t2111: f64, t2117: f64, t57: f64, t1605: f64, t537: f64, t481: f64, t495: f64) -> (f64, f64, f64, f64) {
    let t6045 = t166 * t6044;
    let t6047 = 0.285764e-1_f64 * t759 * t6045;
    let t6062 = 0.1590300183910403919e-2_f64 * t2111 * t122 * t1415 * t57 * t2117;
    let t6063 = t1605 * t537;
    let t6064 = t495 * t481;
    (t6047, t6062, t6063, t6064)
}
