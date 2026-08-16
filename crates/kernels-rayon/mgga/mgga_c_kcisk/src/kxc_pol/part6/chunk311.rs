//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 311/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk311(t662: f64, t695: f64, t661: f64, t657: f64, t667: f64, t1333: f64, t721: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1776 = t662 * t695;
    let t1781 = t661 * t661;
    let t1782 = 1.0_f64 / t1781;
    let t1783 = t657 * t1782;
    let t1791 = 1.0_f64 / t667;
    let t1795 = t1333 * t721;
    let t1796 = 0.16581944444444444444e-2_f64 * t1795;
    let t1797 = 1.0_f64 / t690;
    (t1776, t1781, t1782, t1783, t1791, t1795, t1796, t1797)
}
