//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 904/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk904<F: Float>(t17449: F, t3061: F, t1094: F, t17500: F, t8700: F, t1471: F, t5122: F, t2976: F, t11671: F, t11677: F, t14881: F, t14883: F, t14885: F, t14887: F, t14889: F, t14895: F, t17381: F, t17384: F, t17389: F, t17392: F, t17394: F, t8831: F) -> (F, F, F, F, F, F) {
    let t17787 = t17449 * t3061;
    let t17790 = t17500 * t1094;
    let t17793 = t17449 * t8700;
    let t17802 = t5122 * t1471;
    let t17803 = t17802 * t2976;
    let t17819 = -0.41678000000000000001e0 * t14881 + 0.20839e0 * t14883 + 0.34431666666666666666e0 * t14885 - 0.103295e1 * t14887 + 0.51647499999999999999e0 * t14889 + 0.69463333333333333335e-1 * t14895 + 0.3529725e1 * t17381 + 0.264729375e1 * t17384 - 0.68863333333333333332e0 * t11671 - 0.34731666666666666667e0 * t11677 - 0.104195e0 * t17389 + 0.62517e0 * t17392 - 0.157790625e0 * t17394 - t8831;
    (t17787, t17790, t17793, t17802, t17803, t17819)
}
