//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 615/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk615<F: Float>(t3945: F, t644: F, t647: F, t1432: F, t639: F, t256: F, t1423: F, t1427: F, t1991: F, t3482: F, t519: F, t1498: F, t568: F, t646: F, t695: F, t1198: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3946 = t3945 * t644;
    let t3947 = t3946 * t647;
    let t3949 = t639 * t1432;
    let t3950 = t3949 * t256;
    let t3951 = t1423 * t1427;
    let t3953 = t1991 * t3482;
    let t3955 = 4.0 / 9.0 * t519 * t3953;
    let t3956 = t1498 * t568;
    let t3957 = 4.0 / 15.0 * t3956;
    let t3959 = 0.06649088888888889 * t695 * t646;
    let t3960 = t1198 * t646;
    (t3946, t3947, t3949, t3950, t3951, t3953, t3955, t3956, t3957, t3959, t3960)
}
