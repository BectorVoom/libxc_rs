//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1121/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1121<F: Float>(t325: F, t6651: F, t4606: F, t6654: F, t11: F, t15752: F, t557: F, t331: F, t6824: F, t6827: F, t13665: F, t13667: F, t13675: F, t13677: F, t13679: F, t13681: F, t13705: F) -> (F, F, F, F) {
    let t16432 = t325 * t6651;
    let t16434 = t4606 * t6654;
    let t16437 = t11 * t557 * t15752;
    let t16439 = t331 * t6824;
    let t16441 = t331 * t6827;
    let t16443 = 0.07111111111111111 * t13665 - 0.011851851851851851 * t13667 - 0.017777777777777778 * t13675 - 0.017777777777777778 * t13677 + 0.002962962962962963 * t13679 + 0.003950617283950617 * t13681 + 0.05333333333333334 * t13705 + 0.14396666666666666 * t16432 + 1.0557555555555556 * t16434 - 0.21595 * t16437 - 0.017777777777777778 * t16439 + 0.003950617283950617 * t16441;
    (t16432, t16434, t16437, t16443)
}
