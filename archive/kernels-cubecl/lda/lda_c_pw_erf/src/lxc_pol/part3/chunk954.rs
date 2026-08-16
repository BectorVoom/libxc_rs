//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 954/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk954<F: Float>(t2798: F, t2805: F, t159: F, t285: F, t2853: F, t462: F, t4120: F, t477: F, t1128: F, t1159: F, t2872: F, t695: F) -> (F, F, F, F, F) {
    let t10852 = t2805 * t2798;
    let t10862 = t462 * t2853 * t159 * t285;
    let t10865 = t4120 * t477 * t285;
    let t10868 = t1159 * t1128 * t285;
    let t10872 = F::cast_from(0.0011622696607154768_f64) * t695 * t2872 * t285;
    (t10852, t10862, t10865, t10868, t10872)
}
