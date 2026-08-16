//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2627/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2627(t10777: f64, t10779: f64, t50412: f64, t6035: f64, t4321: f64, t4534: f64, t689: f64, t10995: f64, t18312: f64, t686: f64, t72: f64, t18804: f64, t2470: f64) -> (f64, f64, f64, f64) {
    let t62502 = t10777 * t10779 * t50412 * t6035;
    let t62516 = t689 * t4321 * t4534;
    let t62523 = t10995 * t18312 * t72 * t686;
    let t62528 = t10995 * t18804 * t2470;
    (t62502, t62516, t62523, t62528)
}
