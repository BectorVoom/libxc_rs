//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 972/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk972(t17449: f64, t3061: f64, t1094: f64, t17500: f64, t8700: f64, t1471: f64, t5122: f64, t2976: f64, t11671: f64, t11677: f64, t14881: f64, t14883: f64, t14885: f64, t14887: f64, t14889: f64, t14895: f64, t17381: f64, t17384: f64, t17389: f64, t17392: f64, t17394: f64, t8831: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17787 = t17449 * t3061;
    let t17790 = t17500 * t1094;
    let t17793 = t17449 * t8700;
    let t17802 = t5122 * t1471;
    let t17803 = t17802 * t2976;
    let t17819 = -0.41678000000000000001e0_f64 * t14881 + 0.20839e0_f64 * t14883 + 0.34431666666666666666e0_f64 * t14885 - 0.103295e1_f64 * t14887 + 0.51647499999999999999e0_f64 * t14889 + 0.69463333333333333335e-1_f64 * t14895 + 0.3529725e1_f64 * t17381 + 0.264729375e1_f64 * t17384 - 0.68863333333333333332e0_f64 * t11671 - 0.34731666666666666667e0_f64 * t11677 - 0.104195e0_f64 * t17389 + 0.62517e0_f64 * t17392 - 0.157790625e0_f64 * t17394 - t8831;
    (t17787, t17790, t17793, t17802, t17803, t17819)
}
