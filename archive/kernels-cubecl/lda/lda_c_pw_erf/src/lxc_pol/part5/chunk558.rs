//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 558/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk558<F: Float>(t2998: F, t400: F, t1073: F, t1081: F, t1124: F, t119: F, t84: F, t395: F, t1035: F, t339: F, t1125: F, t31: F, t4: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2999 = t400 * t2998;
    let t3000 = F::cast_from(51.94726769812759_f64) * t2999;
    let t3004 = t1073 * t1081;
    let t3005 = F::cast_from(0.0007324622014701264_f64) * t3004;
    let t3007 = t119 * t1124 * t84;
    let t3008 = t395 * t3007;
    let t3009 = F::cast_from(0.0005696928233656539_f64) * t3008;
    let t3010 = t339 * t1035;
    let t3011 = F::cast_from(12.0_f64) * t3010;
    let t3015 = t4 * t1125 * t31;
    (t2999, t3000, t3004, t3005, t3007, t3008, t3009, t3010, t3011, t3015)
}
