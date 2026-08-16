//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2084/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2084(t4786: f64, t4873: f64, t15584: f64, t11696: f64, t4781: f64, t3092: f64, t11705: f64, t11703: f64, t11678: f64, t357: f64, t1592: f64, t4900: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15585 = t4873 * t4786;
    let t15586 = t15584 * t15585;
    let t15591 = t4781 * t11696;
    let t15592 = t3092 * t15591;
    let t15595 = t4781 * t11705;
    let t15596 = t11703 * t15595;
    let t15599 = t11678 * t357;
    let t15600 = t1592 * t15599;
    let t15601 = t3092 * t15600;
    let t15604 = t4900 * t999;
    (t15585, t15586, t15591, t15592, t15595, t15596, t15599, t15600, t15601, t15604)
}
