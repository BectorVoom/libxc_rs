//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1081/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1081<F: Float>(t14566: F, t14567: F, t14568: F, t14569: F, t14570: F, t8946: F, t8981: F, t8983: F, t8991: F, t8995: F, t8999: F, t117: F, t174: F, t14654: F, t8896: F, t127: F, t3296: F) -> (F, F, F, F) {
    let t14771 = t8946 / 6.0 - 1.46904 * t8981 + 0.73452 * t8983 + t14566 + 5.87616 * t8991 - 2.93808 * t8995 + 5.87616 * t8999 + t14567 - t14568 + t14569 + t14570;
    let t14777 = t117 * t174;
    let t14781 = t8896 * t14654;
    let t14783 = t127 * t3296;
    (t14771, t14777, t14781, t14783)
}
