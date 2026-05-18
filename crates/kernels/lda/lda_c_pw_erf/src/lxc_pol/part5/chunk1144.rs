//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1144/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1144<F: Float>(t504: F, t7797: F, t1325: F, t1326: F, t348: F, t3794: F, t7738: F, t1446: F, t7742: F, t15926: F, t6277: F, t4738: F, t6265: F) -> (F, F, F, F, F) {
    let t21079 = t7797 * t504;
    let t21083 = F::new(8.0) / F::new(45.0) * t1325 * t1326 * t21079 * t348;
    let t21085 = F::new(16.0) / F::new(15.0) * t3794 * t7738;
    let t21087 = F::new(8.0) / F::new(15.0) * t1446 * t7742;
    let t21089 = F::new(8.0) / F::new(15.0) * t15926 * t6277;
    let t21091 = F::new(8.0) / F::new(15.0) * t4738 * t6265;
    (t21083, t21085, t21087, t21089, t21091)
}
