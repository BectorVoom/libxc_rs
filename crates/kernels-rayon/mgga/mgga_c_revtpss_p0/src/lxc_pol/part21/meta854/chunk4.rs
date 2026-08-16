//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3227/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3227(t13043: f64, t1811: f64, t1209: f64, t21451: f64, t1204: f64, t12717: f64, t12753: f64, t12769: f64, t1280: f64, t12975: f64, t13127: f64, t13129: f64, t16757: f64, t17600: f64, t17887: f64, t17888: f64, t17909: f64, t17921: f64, t17941: f64, t3666: f64, t3746: f64, t44843: f64, t45700: f64, t5326: f64, t5452: f64, t5458: f64, t5459: f64, t5466: f64, t56561: f64, t73: f64) -> (f64, f64) {
    let t59784 = t1811 * t13043;
    let t59788 = t1209 * t21451;
    let t59797 = -0.65854491829355115987e0_f64 * t5326 * t12769 + 0.79025390195226139182e1_f64 * t17888 * t16757 + 0.39512695097613069591e1_f64 * t3746 * t17909 - 0.39512695097613069591e1_f64 * t3666 * t17941 - 0.39512695097613069591e1_f64 * t45700 * t5459 + 0.15805078039045227836e2_f64 * t44843 * t1280 * t56561 - 0.19756347548806534796e1_f64 * t12975 * t5452 + 0.79025390195226139182e1_f64 * t1204 * t17887 * t5466 + 0.65854491829355115987e0_f64 * t13127 * t59784 * t13129 - 0.39512695097613069591e1_f64 * t59788 * t12753 + 0.19756347548806534796e1_f64 * t3746 * t17921 + 0.39512695097613069591e1_f64 * t12717 * t17600 * t73 * t5458;
    (t59784, t59797)
}
