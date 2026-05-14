//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1237/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1237<F: Float>(t1234: F, t1316: F, t1317: F, t15136: F, t15168: F, t18044: F, t18056: F, t18087: F, t18095: F, t18127: F, t18151: F, t18410: F, t18440: F, t2180: F, t2236: F, t2255: F, t2311: F, t2733: F, t295: F, t342: F, t4006: F, t4231: F, t4232: F, t4233: F, t5583: F, t5883: F, t61: F, t7089: F, t787: F, t790: F, t8177: F, t8180: F, t8184: F, t8187: F, t8189: F, t8206: F, t8208: F, t8211: F) -> (F,) {
    let t18444 = -t8177 - 9.138438188948293e-06 * t8180 - t8184 + 0.039914113367515366 * t8187 + 0.11974234010254609 * t8189 - 0.01197423401025461 * t8206 + 0.3902713307045947 * t8208 + t8211 - 6.0 * t5583 * t4232 * t2236 * t342 - 6.0 * t5583 * t15136 * t4233 + (t15168 + t18044 + t18056 + t18087) * t61 + 6.0 * t1316 * t2733 * t4006 - 3.0 * t4231 * t18095 + 6.0 * t1316 * t790 * t2255 * t342 + 6.0 * t1316 * t7089 * t1317 + 6.0 * t1316 * t5883 * t2311 + 6.0 * t2180 * t790 * t787 * t1234 + (t18127 + t18151 + t18410 + t18440) * t295;
    (t18444,)
}
