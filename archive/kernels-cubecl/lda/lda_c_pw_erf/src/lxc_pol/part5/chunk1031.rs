//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1031/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1031<F: Float>(t1529: F, t2402: F, t1635: F, t7266: F, t1627: F, t1: F, t3: F, t6039: F, t604: F, t1325: F, t5237: F, t6343: F) -> (F, F, F, F, F) {
    let t17794 = t2402 * t1529;
    let t17814 = t7266 * t1635;
    let t17816 = t7266 * t1627;
    let t17820 = t6039 * t1 * t3 * t604;
    let t17883 = t1325 * t5237 * t6343;
    (t17794, t17814, t17816, t17820, t17883)
}
