//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1509/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1509(t23253: f64, t40348: f64, t10777: f64, t10779: f64, t1559: f64, t5984: f64, t10905: f64, t23275: f64, t6035: f64, t61956: f64, t40725: f64, t5988: f64) -> (f64, f64, f64, f64, f64) {
    let t76647 = t40348 * t23253;
    let t76672 = t10777 * t10779 * t5984 * t1559;
    let t76677 = t10905 * t23275;
    let t76689 = t10777 * t10779 * t61956 * t6035;
    let t76701 = t10777 * t40725 * t5988 * t1559;
    (t76647, t76672, t76677, t76689, t76701)
}
