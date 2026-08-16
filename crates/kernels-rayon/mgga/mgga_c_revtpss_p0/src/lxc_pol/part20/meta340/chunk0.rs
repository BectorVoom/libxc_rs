//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1266/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1266(t12078: f64, t15905: f64, t12160: f64, t4891: f64, t1065: f64, t2852: f64, t2857: f64, t357: f64, t2251: f64, t1014: f64, t140: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15906 = t12078 * t15905;
    let t15917 = t12160 * t4891;
    let t15935 = t1065 * t2852;
    let t15962 = t357 * t2857;
    let t15963 = t15962 * t2251;
    let t15987 = t140 * t1014;
    let t15993 = t140 * t3252;
    (t15906, t15917, t15935, t15963, t15987, t15993)
}
