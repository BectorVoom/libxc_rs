//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 868/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk868<F: Float>(t11381: F, t69: F, t11404: F, t11392: F, t11308: F, t11311: F, t11318: F, t11336: F, t11344: F, t11377: F, t11380: F, t11385: F, t11408: F, t2247: F, t8455: F, t11391: F, t11401: F, t11403: F, t11413: F, t11426: F, t11427: F, t11430: F, t11431: F, t11436: F, t11437: F, t11441: F, t11443: F, t11444: F, t8339: F) -> (F, F) {
    let t11515 = t69 * t11381;
    let t11519 = t69 * t11404;
    let t11521 = t69 * t11392;
    let t11524 = -62.07318 * t2247 * t11408 - t11308 - t11311 + t11318 + t11336 - t11344 + 1.724255 * t11515 - 1.724255 * t69 * t11385 - 2.2990066666666666 * t11519 + 1.7881162962962962 * t11521 - 5.172765 * t8455 + t11377 + t11380;
    let t11525 = t11391 - t11401 - t11403 + t11413 + t11426 - t11427 - t11430 + t11431 + t11436 - t8339 + t11437 + t11441 + t11443 - t11444;
    (t11524, t11525)
}
