//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2546/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2546(t15711: f64, t3106: f64, t15935: f64, t372: f64, t15904: f64, t245: f64, t3088: f64, t12167: f64, t1063: f64, t1592: f64, t247: f64, t42778: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53724 = t3106 * t15711;
    let t53728 = t372 * t15935;
    let t53739 = t15904 * t245;
    let t53740 = t3088 * t53739;
    let t53741 = t12167 * t53740;
    let t53762 = t1063 * t247 * t42778 * t1592;
    (t53724, t53728, t53739, t53740, t53741, t53762)
}
