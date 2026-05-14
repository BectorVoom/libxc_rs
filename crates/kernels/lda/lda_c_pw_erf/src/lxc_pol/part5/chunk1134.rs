//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1134/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1134<F: Float>(t21544: F, t21549: F, t21551: F, t21553: F, t21554: F, t21555: F, t21556: F, t21557: F, t21561: F, t21564: F, t21568: F, t21570: F, t21571: F, t17432: F, t21573: F, t21575: F, t21576: F, t21581: F, t21582: F, t21587: F, t21591: F, t21596: F, t21601: F, t21605: F, t21608: F, t21611: F) -> (F, F) {
    let t23253 = -t21544 - t21549 - t21551 - t21553 - t21554 - t21555 - t21556 + t21557 + t21561 - t21564 + t21568 + t21570 - t21571;
    let t23255 = 4.0 * t17432 + t21573 + t21575 + t21576 - t21581 - t21582 + t21587 - t21591 + t21596 + t21601 + t21605 + t21608 - t21611;
    (t23253, t23255)
}
