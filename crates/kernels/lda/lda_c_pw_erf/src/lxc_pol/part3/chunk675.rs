//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 675/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk675<F: Float>(t1726: F, t405: F, t1112: F, t462: F, t159: F, t285: F, t1159: F, t477: F, t1128: F, t695: F, t39: F, t465: F) -> (F, F, F, F, F, F) {
    let t4117 = t405 * t1726;
    let t4120 = t462 * t1112;
    let t4122 = t4120 * t159 * t285;
    let t4125 = t1159 * t477 * t285;
    let t4129 = F::new(0.0008717022455366076) * t695 * t1128 * t285;
    let t4130 = t39 * t465;
    (t4117, t4120, t4122, t4125, t4129, t4130)
}
