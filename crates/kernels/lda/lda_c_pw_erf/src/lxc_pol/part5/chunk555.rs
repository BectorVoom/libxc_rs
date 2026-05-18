//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 555/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk555<F: Float>(t1030: F, t2940: F, t2946: F, t400: F, t1059: F, t1077: F, t659: F, t661: F, t1026: F, t378: F) -> (F, F, F, F, F, F, F, F) {
    let t2948 = t2946 * t2940 * t1030;
    let t2949 = t400 * t2948;
    let t2950 = F::new(103.89453539625518) * t2949;
    let t2951 = t1059 * t1077;
    let t2952 = F::new(3.5089340384731225) * t2951;
    let t2953 = F::new(1.0) / t659;
    let t2966 = F::new(1.0) / t661;
    let t2983 = F::new(1.0) / t1026 / t378;
    (t2948, t2949, t2950, t2951, t2952, t2953, t2966, t2983)
}
