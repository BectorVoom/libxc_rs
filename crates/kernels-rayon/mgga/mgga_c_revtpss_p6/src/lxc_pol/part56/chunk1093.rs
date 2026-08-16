//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1093/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1093(t125559: f64, t25082: f64, t8717: f64, t1907: f64, t7311: f64, t28196: f64, t28197: f64, t120967: f64, t1399: f64, t1868: f64, t247: f64, t561: f64) -> (f64, f64, f64) {
    let t125562 = 6.0_f64 * t25082 * t8717 * t125559;
    let t125563 = t1907 * t7311;
    let t125566 = 4.0_f64 * t28196 * t28197 * t125563;
    let t125570 = t120967 * t247 * t561 * t1868 * t1399;
    (t125562, t125566, t125570)
}
