//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 972/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk972<F: Float>(t17449: F, t3061: F, t1094: F, t17500: F, t8700: F, t1471: F, t5122: F, t2976: F, t11671: F, t11677: F, t14881: F, t14883: F, t14885: F, t14887: F, t14889: F, t14895: F, t17381: F, t17384: F, t17389: F, t17392: F, t17394: F, t8831: F) -> (F, F, F, F, F, F) {
    let t17787 = t17449 * t3061;
    let t17790 = t17500 * t1094;
    let t17793 = t17449 * t8700;
    let t17802 = t5122 * t1471;
    let t17803 = t17802 * t2976;
    let t17819 = -F::cast_from(0.41678000000000000001e0_f64) * t14881 + F::new(0.20839e0) * t14883 + F::cast_from(0.34431666666666666666e0_f64) * t14885 - F::new(0.103295e1) * t14887 + F::cast_from(0.51647499999999999999e0_f64) * t14889 + F::cast_from(0.69463333333333333335e-1_f64) * t14895 + F::new(0.3529725e1) * t17381 + F::cast_from(0.264729375e1_f64) * t17384 - F::cast_from(0.68863333333333333332e0_f64) * t11671 - F::cast_from(0.34731666666666666667e0_f64) * t11677 - F::new(0.104195e0) * t17389 + F::new(0.62517e0) * t17392 - F::cast_from(0.157790625e0_f64) * t17394 - t8831;
    (t17787, t17790, t17793, t17802, t17803, t17819)
}
