//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 207/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk207(t12: f64, t2: f64, t387: f64, t390: f64, t637: f64) -> (f64, f64, f64, f64) {
    let t639 = 1.0_f64/f64::sqrt(t12);
    let t640 = t639 * t2;
    let t641 = t640 * t387;
    let t644 = 0.25319e1_f64 * t637 - 0.204775e0_f64 * t641 - 0.82156666666666666667e-1_f64 * t390;
    (t639, t640, t641, t644)
}
