//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 795/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk795(t159: f64, t7778: f64, t617: f64, t1678: f64, t955: f64, t1686: f64, t2035: f64, t898: f64, t41: f64, t5883: f64, t5885: f64, t1745: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7779 = t159 * t7778;
    let t7781 = 0.16936279733333333333e-2_f64 * t7779 * t617;
    let t7783 = t955 * t1678;
    let t7784 = t159 * t7783;
    let t7785 = t7784 * t1686;
    let t7794 = t898 * t2035;
    let t7795 = t41 * t7794;
    let t7796 = 4.0_f64 * t5883;
    let t7797 = 12.0_f64 * t5885;
    let t7798 = t963 * t1745;
    (t7781, t7785, t7795, t7796, t7797, t7798)
}
