//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1355/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1355<F: Float>(t14655: F, t14657: F, t19653: F, t14781: F, t14783: F, t19636: F, t19639: F, t19640: F, t19641: F, t19642: F, t19643: F, t19646: F, t19648: F, t19651: F, t127: F, t14787: F, t14795: F, t14797: F, t14799: F, t14802: F, t14807: F, t14813: F, t14816: F, t14819: F, t14822: F, t14837: F, t1697: F, t18826: F, t19097: F, t436: F) -> (F, F, F, F) {
    let t19656 = 3.8973666666666666 * t14655;
    let t19658 = 70.1526 * t14657 * t19653;
    let t19659 = -t19636 + t19639 - t19640 - t19641 - t19642 + t19643 + t19646 - t19648 - t19651 - 1.95872 * t14781 - 117.5232 * t14783 * t19653 + t19656 - t19658;
    let t19677 = 29.3808 * t14787 - 3.91744 * t14795 + 3.91744 * t14797 + 11.75232 * t127 * t1697 * t18826 - 1.46904 * t127 * t436 * t19097 - 5.87616 * t14799 + 7.83488 * t14802 + 4.0 * t14807 + 15.66976 * t14813 + 8.0 / 3.0 * t14816 - 2.0 * t14819 - t14822 - 11.75232 * t14837;
    (t19656, t19658, t19659, t19677)
}
