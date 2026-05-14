//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1269/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1269<F: Float>(t169: F, t6040: F, t632: F, t1143: F, t2364: F, t15332: F, t15333: F, t15334: F, t15335: F, t15336: F, t15337: F, t15338: F, t15339: F, t15340: F, t15342: F, t15345: F, t15347: F, t15350: F, t15406: F, t8168: F, t8177: F, t8188: F) -> (F, F, F) {
    let t18942 = t169 * t6040 * t632;
    let t18945 = t169 * t2364 * t1143;
    let t18956 = t15332 - t15333 - t8168 - t8177 - t15334 - t15335 - t15336 - t15337 + t15338 + t15339 + t15340 - t15342 - t15345 - t15347 + t15350 + t15406 - t8188;
    (t18942, t18945, t18956)
}
