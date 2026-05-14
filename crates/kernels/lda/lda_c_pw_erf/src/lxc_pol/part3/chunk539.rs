//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 539/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk539<F: Float>(t3004: F, t1124: F, t119: F, t84: F, t395: F, t1035: F, t339: F, t2761: F, t2944: F, t2950: F, t2952: F, t2981: F, t2989: F, t2991: F, t2995: F, t3000: F, t3003: F) -> (F, F, F, F, F) {
    let t3005 = 0.0007324622014701264 * t3004;
    let t3007 = t119 * t1124 * t84;
    let t3008 = t395 * t3007;
    let t3009 = 0.0005696928233656539 * t3008;
    let t3010 = t339 * t1035;
    let t3011 = 12.0 * t3010;
    let t3012 = -t2761 - t2944 + t2950 + t2952 + t2981 - t2989 - t2991 + t2995 - t3000 - t3003 + t3005 - t3009 + t3011;
    (t3007, t3008, t3009, t3010, t3012)
}
