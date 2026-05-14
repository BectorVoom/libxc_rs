//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 589/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk589<F: Float>(t188: F, t3675: F, t1392: F, t542: F, t186: F, t185: F, t1217: F, t665: F, t1231: F, t668: F, t348: F, t92: F, t352: F, t93: F, t108: F, t2954: F, t2961: F, t2967: F, t2973: F, t406: F, t408: F, t659: F, t661: F, t945: F, t954: F) -> (F, F, F, F, F, F, F) {
    let t3676 = t188 * t3675;
    let t3677 = t1392 * t542;
    let t3678 = t3676 * t3677;
    let t3679 = t186 * t3678;
    let t3681 = 4.0 / 5.0 * t185 * t3679;
    let t3682 = t665 * t1217;
    let t3684 = t1231 * t668;
    let t3688 = t92 * t348;
    let t3695 = t93 * t352;
    let t3701 = (40.0 / 27.0 * t406 * t2954 + 20.0 / 3.0 * t3688 * t945 + 4.0 / 3.0 * t659 * t2961 + 40.0 / 27.0 * t408 * t2967 + 20.0 / 3.0 * t3695 * t954 + 4.0 / 3.0 * t661 * t2973) * t108;
    (t3677, t3678, t3679, t3681, t3682, t3684, t3701)
}
