//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 913/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk913<F: Float>(t1446: F, t4834: F, t5234: F, t5238: F, t4804: F, t5276: F, t3794: F, t3476: F, t5146: F, t11857: F, t4488: F, t11999: F, t12001: F, t12003: F, t12008: F, t12010: F, t12012: F, t12014: F) -> (F, F, F, F, F, F, F, F) {
    let t12015 = t1446 * t4834;
    let t12016 = 16.0 / 45.0 * t12015;
    let t12017 = t1446 * t5234;
    let t12018 = 32.0 / 45.0 * t12017;
    let t12019 = t1446 * t5238;
    let t12020 = 16.0 / 27.0 * t12019;
    let t12022 = 8.0 / 15.0 * t4804 * t5276;
    let t12024 = 8.0 / 15.0 * t3794 * t5276;
    let t12025 = t5146 * t3476;
    let t12028 = 8.0 / 3.0 * t4488 * t12025 * t11857;
    let t12029 = t11999 + t12001 + t12003 + t12008 - t12010 + t12012 - t12014 - t12016 - t12018 + t12020 + t12022 + t12024 - t12028;
    (t12016, t12018, t12020, t12022, t12024, t12025, t12028, t12029)
}
