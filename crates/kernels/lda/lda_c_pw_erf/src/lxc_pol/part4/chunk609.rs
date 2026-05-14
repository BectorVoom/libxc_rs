//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 609/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk609<F: Float>(t2940: F, t2983: F, t2986: F, t400: F, t1055: F, t1059: F, t1010: F, t1022: F, t387: F, t1030: F, t385: F, t1027: F, t1: F, t960: F, t397: F, t1073: F, t1081: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2987 = t2983 * t2940 * t2986;
    let t2988 = t400 * t2987;
    let t2989 = 1025.3897021007795 * t2988;
    let t2990 = t1059 * t1055;
    let t2993 = t1010 * t1022 * t387;
    let t2994 = t400 * t2993;
    let t2995 = 3.5089340384731225 * t2994;
    let t2997 = t1030 * t385;
    let t2998 = t1027 * t1022 * t2997;
    let t2999 = t400 * t2998;
    let t3000 = 51.94726769812759 * t2999;
    let t3001 = t960 * t1;
    let t3002 = t3001 * t397;
    let t3004 = t1073 * t1081;
    (t2987, t2988, t2989, t2990, t2993, t2994, t2995, t2997, t2998, t2999, t3000, t3001, t3002, t3004)
}
