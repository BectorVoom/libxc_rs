//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 911/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk911(t2271: f64, t3162: f64, t372: f64, t7048: f64, t7050: f64, t7095: f64, t7097: f64, t8644: f64, t8646: f64, t8647: f64, t881: f64, t9005: f64, t9063: f64, t9066: f64, t9592: f64) -> f64 {
    let t9804 = t2271 * t3162;
    let t9810 = t372 * t9005 - t7048 - t7050 + t8644 + t9592 + t7095 + t7097 - 0.2363e1_f64 * t9804 - 0.2363e1_f64 * t881 * t9063 - 0.2363e1_f64 * t881 * t9066 - t8646 + t8647;
    t9810
}
