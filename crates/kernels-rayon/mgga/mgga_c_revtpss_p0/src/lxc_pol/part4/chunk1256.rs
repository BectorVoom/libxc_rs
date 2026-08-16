//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1256/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1256(t11705: f64, t4781: f64, t11703: f64, t11678: f64, t357: f64, t1592: f64, t3092: f64, t4900: f64, t999: f64, t4893: f64, t3117: f64, t4894: f64) -> (f64, f64, f64, f64, f64) {
    let t15595 = t4781 * t11705;
    let t15596 = t11703 * t15595;
    let t15599 = t11678 * t357;
    let t15600 = t1592 * t15599;
    let t15601 = t3092 * t15600;
    let t15604 = t4900 * t999;
    let t15605 = t4893 * t15604;
    let t15606 = t3117 * t15605;
    let t15609 = t4894 * t999;
    (t15596, t15601, t15604, t15606, t15609)
}
