//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 357/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk357(t604: f64, t662: f64, t695: f64, t1060: f64, t1775: f64, t661: f64, t657: f64, t1689: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t659 = 0.0_f64 < t604;
    let t1776 = t662 * t695;
    let t1777 = t1776 * t1060;
    let t1778 = t1775 * t1777;
    let t1781 = t661 * t661;
    let t1782 = 1.0_f64 / t1781;
    let t1783 = t657 * t1782;
    let t1785 = piecewise3(t659, t1689, -t1689);
    (t1776, t1777, t1778, t1781, t1782, t1783, t1785)
}
