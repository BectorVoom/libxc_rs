//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 902/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk902(t3309: f64, t3310: f64, t343: f64, t3318: f64, t3319: f64, t1687: f64, t5021: f64, t1653: f64, t432: f64, t4606: f64, t416: f64, t118: f64, t1184: f64, t119: f64, t120: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8862 = 2.6116266666666665_f64 * t3309 * t3310 * t343;
    let t8865 = 15.589466666666667_f64 * t3318 * t3319 * t343;
    let t8867 = 2.9018074074074076_f64 * t1687 * t5021;
    let t8869 = 5.773876543209877_f64 * t1653 * t5021;
    let t8871 = 2.5390814814814813_f64 * t432 * t4606;
    let t8873 = 5.052141975308642_f64 * t416 * t4606;
    let t8877 = 70.0_f64 / 81.0_f64 * t118 * t119 * t1184 * t120;
    (t8862, t8865, t8867, t8869, t8871, t8873, t8877)
}
