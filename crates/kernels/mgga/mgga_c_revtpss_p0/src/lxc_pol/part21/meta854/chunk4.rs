//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3227/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3227<F: Float>(t13043: F, t1811: F, t1209: F, t21451: F, t1204: F, t12717: F, t12753: F, t12769: F, t1280: F, t12975: F, t13127: F, t13129: F, t16757: F, t17600: F, t17887: F, t17888: F, t17909: F, t17921: F, t17941: F, t3666: F, t3746: F, t44843: F, t45700: F, t5326: F, t5452: F, t5458: F, t5459: F, t5466: F, t56561: F, t73: F) -> (F, F) {
    let t59784 = t1811 * t13043;
    let t59788 = t1209 * t21451;
    let t59797 = -F::cast_from(0.65854491829355115987e0_f64) * t5326 * t12769 + F::cast_from(0.79025390195226139182e1_f64) * t17888 * t16757 + F::cast_from(0.39512695097613069591e1_f64) * t3746 * t17909 - F::cast_from(0.39512695097613069591e1_f64) * t3666 * t17941 - F::cast_from(0.39512695097613069591e1_f64) * t45700 * t5459 + F::cast_from(0.15805078039045227836e2_f64) * t44843 * t1280 * t56561 - F::cast_from(0.19756347548806534796e1_f64) * t12975 * t5452 + F::cast_from(0.79025390195226139182e1_f64) * t1204 * t17887 * t5466 + F::cast_from(0.65854491829355115987e0_f64) * t13127 * t59784 * t13129 - F::cast_from(0.39512695097613069591e1_f64) * t59788 * t12753 + F::cast_from(0.19756347548806534796e1_f64) * t3746 * t17921 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t17600 * t73 * t5458;
    (t59784, t59797)
}
