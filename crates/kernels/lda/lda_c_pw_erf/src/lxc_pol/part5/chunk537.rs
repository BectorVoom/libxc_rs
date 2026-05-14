//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 537/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk537<F: Float>(t3007: F, t395: F, t1035: F, t339: F, t1125: F, t31: F, t4: F, t1034: F, t357: F, t40: F, t379: F, t473: F, t1027: F, t155: F, t364: F, t988: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3008 = t395 * t3007;
    let t3009 = 0.0005696928233656539 * t3008;
    let t3010 = t339 * t1035;
    let t3011 = 12.0 * t3010;
    let t3015 = t4 * t1125 * t31;
    let t3016 = 0.0034451131037037037 * t3015;
    let t3017 = t357 * t1034;
    let t3018 = t40 * t3017;
    let t3019 = 3.0 * t3018;
    let t3020 = t473 * t379;
    let t3027 = t155 * t1027;
    let t3031 = t473 * t364;
    let t3038 = t155 * t988;
    (t3008, t3009, t3010, t3011, t3015, t3016, t3017, t3018, t3019, t3020, t3027, t3031, t3038)
}
