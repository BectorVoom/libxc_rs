//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2947/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2947(t15905: f64, t43420: f64, t43574: f64, t11922: f64, t15781: f64, t4892: f64, t42865: f64, t72: f64, t3088: f64, t43472: f64, t1668: f64, t42871: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53654 = t43420 * t15905;
    let t53657 = t43574 * t15905;
    let t53661 = t4892 * t11922 * t15781;
    let t53667 = t42865 * t72;
    let t53668 = t3088 * t53667;
    let t53669 = t43472 * t53668;
    let t53670 = t1668 * t42871;
    (t53654, t53657, t53661, t53667, t53668, t53669, t53670)
}
