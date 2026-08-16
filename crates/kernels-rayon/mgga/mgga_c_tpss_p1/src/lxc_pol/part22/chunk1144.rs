//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1144/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1144(t9959: f64, t9961: f64, t9963: f64, t9966: f64, t2345: f64, t4438: f64, t4397: f64, t541: f64, t3234: f64, t4533: f64, t177: f64, t4377: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12754 = 12.0_f64 * t9959;
    let t12755 = 4.0_f64 * t9961;
    let t12756 = 4.0_f64 * t9963;
    let t12757 = 80.0_f64 * t9966;
    let t12758 = t4438 * t2345;
    let t12759 = 0.11696447245269292414e1_f64 * t12758;
    let t12760 = t541 * t4397;
    let t12764 = t4533 * t3234;
    let t12767 = t4377 * t177;
    (t12754, t12755, t12756, t12757, t12759, t12760, t12764, t12767)
}
