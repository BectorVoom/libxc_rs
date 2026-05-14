//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1230/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1230<F: Float>(t1472: F, t6447: F, t12881: F, t2393: F, t12641: F, t1446: F, t6489: F, t1313: F, t5127: F, t519: F, t789: F, t4804: F, t6455: F, t10474: F, t3794: F, t1278: F, t2433: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18257 = 64.0 / 45.0 * t1472 * t6447;
    let t18259 = 16.0 / 45.0 * t12881 * t2393;
    let t18261 = 32.0 / 45.0 * t12641 * t2393;
    let t18263 = 16.0 / 45.0 * t1446 * t6489;
    let t18267 = 8.0 / 45.0 * t519 * t1313 * t789 * t5127;
    let t18269 = 64.0 / 45.0 * t4804 * t6455;
    let t18271 = 16.0 / 45.0 * t10474 * t2393;
    let t18273 = 64.0 / 45.0 * t3794 * t6455;
    let t18277 = 8.0 / 45.0 * t519 * t1313 * t2433 * t1278;
    (t18257, t18259, t18261, t18263, t18267, t18269, t18271, t18273, t18277)
}
