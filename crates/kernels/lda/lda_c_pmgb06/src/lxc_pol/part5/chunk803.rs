//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 803/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk803<F: Float>(t1380: F, t7538: F, t493: F, t1897: F, t7493: F, t439: F, t7470: F, t7472: F, t7505: F, t7506: F, t7507: F, t7511: F, t7515: F, t7519: F, t7523: F, t7527: F, t7529: F, t7531: F, t7534: F, t7537: F) -> (F, F, F, F, F) {
    let t7539 = t1380 * t7538;
    let t7541 = t493 * t7539 / F::new(15.0);
    let t7542 = t1897 * t7493;
    let t7544 = F::new(2.0) / F::new(15.0) * t439 * t7542;
    let t7545 = t7470 + t7472 + t7505 - t7506 - t7507 - t7511 - t7515 + t7519 - t7523 - t7527 - t7529 - t7531 - t7534 - t7537 - t7541 - t7544;
    (t7539, t7541, t7542, t7544, t7545)
}
