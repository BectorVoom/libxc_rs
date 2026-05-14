//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1091/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1091<F: Float>(t16309: F, t439: F, t4766: F, t6550: F, t2477: F, t3213: F, t10687: F, t10690: F, t16293: F, t16295: F, t16297: F, t16299: F, t16300: F, t16301: F, t16302: F, t16303: F, t16306: F, t16308: F) -> (F, F, F, F) {
    let t16310 = 8.0 / 45.0 * t16309;
    let t16313 = 2.0 / 5.0 * t439 * t6550 * t4766;
    let t16314 = t3213 * t2477;
    let t16315 = 4.0 / 405.0 * t16314;
    let t16316 = -t16293 - t16295 - t16297 - t16299 + t16300 + t16301 + t16302 + t16303 - t16306 - t10687 + t10690 - t16308 + t16310 - t16313 - t16315;
    (t16310, t16313, t16315, t16316)
}
