//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1057/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1057<F: Float>(t15376: F, t15402: F, t59: F, t40: F, t87: F, t11305: F, t15332: F, t15333: F, t15334: F, t15335: F, t15336: F, t15337: F, t15338: F, t15339: F, t15340: F, t15342: F, t15345: F, t15347: F, t15350: F, t8168: F, t8177: F, t8184: F, t8188: F) -> (F, F, F) {
    let t15404 = (t15376 + t15402) * t59;
    let t15406 = t40 * t15404 * t87;
    let t15407 = -0.9480012043054112 * t11305 + t15332 - t15333 - t8168 - t8177 - t15334 - t15335 - t15336 - t15337 + t15338 + t15339 + t15340 + t8184 - t15342 - t15345 - t15347 + t15350 + t15406 - t8188;
    (t15404, t15406, t15407)
}
