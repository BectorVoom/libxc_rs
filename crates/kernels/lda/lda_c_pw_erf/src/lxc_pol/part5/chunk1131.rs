//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1131/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1131<F: Float>(t12685: F, t12709: F, t21338: F, t21342: F, t21344: F, t21346: F, t21349: F, t21351: F, t21353: F, t21355: F, t21358: F, t21360: F, t21363: F, t12714: F, t21364: F, t21365: F, t21366: F, t21378: F, t21381: F, t21384: F, t21385: F, t21387: F, t21388: F, t21389: F, t21390: F, t21392: F) -> (F, F) {
    let t23240 = -t12685 - t12709 + t21338 + t21342 - t21344 + t21346 + t21349 + t21351 + t21353 - t21355 - t21358 - t21360 + t21363;
    let t23242 = t21364 + t21365 + t21366 + 0.0033101111111111113 * t12714 - t21378 - t21381 - t21384 - t21385 + t21387 + t21388 + t21389 + t21390 - t21392;
    (t23240, t23242)
}
