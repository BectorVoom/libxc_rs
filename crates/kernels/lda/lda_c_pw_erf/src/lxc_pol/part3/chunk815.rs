//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 815/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk815<F: Float>(t3327: F, t443: F, t1704: F, t1710: F, t3338: F, t440: F, t131: F, t137: F, t3337: F, t120: F, t133: F, t2869: F, t1870: F, t3357: F, t5639: F, t8939: F) -> (F, F, F, F, F, F, F) {
    let t9051 = t3327 * t443;
    let t9054 = t1704 * t1710;
    let t9059 = t440 * t3338;
    let t9068 = t131 / t3337 / t137;
    let t9083 = 2.9801938271604937 * t133 * t2869 * t120;
    let t9094 = t1870 * t5639 * t3357;
    let t9096 = t133 * t8939;
    (t9051, t9054, t9059, t9068, t9083, t9094, t9096)
}
