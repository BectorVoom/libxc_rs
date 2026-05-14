//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1051/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1051<F: Float>(t18188: F, t2006: F, t3965: F, t16657: F, t1996: F, t4488: F, t17883: F, t17886: F, t1308: F, t571: F, t593: F, t7422: F, t1319: F, t21777: F, t2017: F, t21794: F) -> (F, F, F, F, F, F, F) {
    let t21885 = 8.0 / 15.0 * t3965 * t18188 * t2006;
    let t21888 = 8.0 / 15.0 * t4488 * t16657 * t1996;
    let t21889 = 16.0 / 27.0 * t17883;
    let t21890 = 16.0 / 45.0 * t17886;
    let t21894 = 8.0 / 15.0 * t571 * t1308 * t7422 * t593;
    let t21897 = 32.0 / 15.0 * t571 * t1319 * t21777;
    let t21900 = 16.0 / 3.0 * t571 * t2017 * t21794;
    (t21885, t21888, t21889, t21890, t21894, t21897, t21900)
}
