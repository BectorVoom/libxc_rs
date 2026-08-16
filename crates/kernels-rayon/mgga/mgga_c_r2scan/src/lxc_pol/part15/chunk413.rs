//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 413/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk413(t166: f64, t1678: f64, t159: f64, t15: f64, t26: f64, t20: f64, t21: f64, t263: f64, t695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1679 = t166 * t1678;
    let t1680 = t159 * t1679;
    let t1683 = 1.0_f64 / t15 / t26 / 4.0_f64;
    let t1684 = t1683 * t20;
    let t1685 = t21 * t263;
    let t1686 = t1684 * t1685;
    let t1688 = 0.42340699333333333333e-3_f64 * t1680 * t1686;
    let t1691 = t695 * t695;
    (t1679, t1680, t1683, t1684, t1685, t1686, t1688, t1691)
}
