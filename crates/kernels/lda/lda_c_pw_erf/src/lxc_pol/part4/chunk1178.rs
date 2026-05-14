//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1178/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1178<F: Float>(t12908: F, t12918: F, t12924: F, t14030: F, t3974: F, t4819: F, t13115: F, t14034: F, t2022: F, t4693: F, t6748: F, t565: F, t6297: F, t12942: F, t12944: F, t2127: F, t5215: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17401 = 16.0 / 135.0 * t12908;
    let t17402 = 32.0 / 45.0 * t12918;
    let t17403 = 16.0 / 81.0 * t12924;
    let t17406 = 16.0 / 27.0 * t3974 * t14030 * t4819;
    let t17409 = 32.0 / 45.0 * t13115 * t14034 * t2022;
    let t17412 = 32.0 / 45.0 * t3974 * t6748 * t4693;
    let t17413 = t565 * t6297;
    let t17414 = 8.0 / 45.0 * t17413;
    let t17415 = 16.0 / 45.0 * t12942;
    let t17416 = 32.0 / 45.0 * t12944;
    let t17417 = t5215 * t2127;
    (t17401, t17402, t17403, t17406, t17409, t17412, t17414, t17415, t17416, t17417)
}
