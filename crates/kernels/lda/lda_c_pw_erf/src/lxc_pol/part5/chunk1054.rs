//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1054/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1054<F: Float>(t1318: F, t34: F, t4892: F, t6188: F, t4753: F, t7570: F, t3416: F, t3899: F, t7596: F, t13479: F, t21915: F, t21917: F, t21919: F, t21921: F, t21923: F, t21926: F, t21928: F, t21932: F) -> (F, F, F, F, F) {
    let t21936 = 4.0 / 5.0 * t1318 * t4892 * t6188 * t34;
    let t21938 = 4.0 / 5.0 * t4753 * t7570;
    let t21940 = 4.0 / 5.0 * t3416 * t7570;
    let t21942 = t1318 * t3899 * t7596;
    let t21943 = 8.0 / 15.0 * t21942;
    let t21944 = -t21915 + t21917 - t21919 - t21921 + t21923 - t21926 + t21928 - t13479 - t21932 + t21936 - t21938 - t21940 - t21943;
    (t21936, t21938, t21940, t21943, t21944)
}
