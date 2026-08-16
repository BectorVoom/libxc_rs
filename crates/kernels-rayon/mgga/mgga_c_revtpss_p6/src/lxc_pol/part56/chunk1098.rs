//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1098/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1098(t121116: f64, t33926: f64, t121011: f64, t1399: f64, t1426: f64, t1903: f64, t247: f64, t1882: f64, t94396: f64, t125627: f64, t1892: f64, t31805: f64) -> (f64, f64, f64, f64, f64) {
    let t125632 = t121116 * t33926;
    let t125637 = t121011 * t247 * t1426 * t1903 * t1399;
    let t125639 = t1426 * t1882;
    let t125642 = t121011 * t247 * t125639 * t94396;
    let t125646 = t121011 * t247 * t125627 * t1399;
    let t125648 = t31805 * t1892;
    (t125632, t125637, t125642, t125646, t125648)
}
