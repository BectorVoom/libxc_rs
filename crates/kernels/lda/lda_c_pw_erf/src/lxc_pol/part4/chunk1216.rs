//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1216/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1216<F: Float>(t1336: F, t7007: F, t4763: F, t4943: F, t1318: F, t3899: F, t6189: F, t10030: F, t6753: F, t3974: F, t4475: F, t4837: F, t13115: F, t15721: F, t5160: F, t13771: F, t15708: F, t4515: F) -> (F, F, F, F, F, F, F) {
    let t18019 = 16.0 / 45.0 * t7007 * t1336;
    let t18021 = 16.0 / 15.0 * t4763 * t4943;
    let t18023 = t1318 * t3899 * t6189;
    let t18024 = 16.0 / 45.0 * t18023;
    let t18025 = t10030 * t6753;
    let t18026 = 64.0 / 81.0 * t18025;
    let t18029 = 16.0 / 45.0 * t3974 * t4475 * t4837;
    let t18032 = 128.0 / 45.0 * t13115 * t5160 * t15721;
    let t18035 = 64.0 / 45.0 * t13771 * t4515 * t15708;
    (t18019, t18021, t18024, t18026, t18029, t18032, t18035)
}
