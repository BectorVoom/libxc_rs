//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 626/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk626<F: Float>(t188: F, t3675: F, t1392: F, t542: F, t186: F, t185: F, t1217: F, t665: F, t1231: F, t668: F, t348: F, t92: F) -> (F, F, F, F, F, F, F) {
    let t3676 = t188 * t3675;
    let t3677 = t1392 * t542;
    let t3678 = t3676 * t3677;
    let t3679 = t186 * t3678;
    let t3681 = F::new(4.0) / F::new(5.0) * t185 * t3679;
    let t3682 = t665 * t1217;
    let t3684 = t1231 * t668;
    let t3688 = t92 * t348;
    (t3677, t3678, t3679, t3681, t3682, t3684, t3688)
}
