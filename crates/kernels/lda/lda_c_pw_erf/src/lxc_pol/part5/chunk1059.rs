//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1059/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1059<F: Float>(t159: F, t285: F, t462: F, t6039: F, t477: F, t6138: F, t1191: F, t169: F, t2357: F, t301: F, t2363: F, t39: F) -> (F, F, F, F) {
    let t19847 = t462 * t6039 * t159 * t285;
    let t19850 = t6138 * t477 * t285;
    let t19860 = t169 * t1191 * t2357 * t301;
    let t19864 = t39 * t2363 * t159 * t285;
    (t19847, t19850, t19860, t19864)
}
