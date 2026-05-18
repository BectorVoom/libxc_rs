//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 714/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk714<F: Float>(t1472: F, t2389: F, t2065: F, t816: F, t1308: F, t571: F, t1954: F, t833: F, t4841: F, t2415: F, t549: F, t1319: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6395 = F::new(8.0) / F::new(45.0) * t1472 * t2389;
    let t6396 = t816 * t2065;
    let t6397 = t1308 * t6396;
    let t6399 = F::new(8.0) / F::new(45.0) * t571 * t6397;
    let t6400 = t1954 * t833;
    let t6401 = t4841 * t6400;
    let t6403 = F::new(16.0) / F::new(45.0) * t571 * t6401;
    let t6404 = t2415 * t549;
    let t6405 = t1319 * t6404;
    (t6395, t6396, t6397, t6399, t6400, t6401, t6403, t6404, t6405)
}
