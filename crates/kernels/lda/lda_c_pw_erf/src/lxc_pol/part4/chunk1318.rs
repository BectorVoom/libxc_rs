//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1318/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1318<F: Float>(t17605: F, t17606: F, t17608: F, t17609: F, t17610: F, t17611: F, t17612: F, t17613: F, t17614: F, t17615: F, t17616: F, t17617: F, t17618: F, t17619: F, t17628: F, t17632: F, t17636: F) -> (F,) {
    let t19264 = -t17605 - t17606 - t17608 - t17609 + t17610 + t17611 + t17612 + t17613 + t17614 - t17615 + t17616 - t17617 + t17618 + t17619 + t17628 - t17632 + t17636;
    (t19264,)
}
