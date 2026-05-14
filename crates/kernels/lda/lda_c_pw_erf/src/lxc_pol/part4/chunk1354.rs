//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1354/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1354<F: Float>(t102: F, t1664: F, t2627: F, t2624: F, t14640: F, t14643: F, t14647: F, t14651: F, t2615: F, t415: F, t5594: F, t19583: F, t5607: F, t2619: F, t1832: F, t1872: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19636 = 17.53815 * t102 * t2627 * t1664;
    let t19639 = 70.1526 * t102 * t2624 * t1664;
    let t19640 = 2.5982444444444446 * t14640;
    let t19641 = 3.8973666666666666 * t14643;
    let t19642 = 1.9486833333333333 * t14647;
    let t19643 = 5.196488888888889 * t14651;
    let t19645 = t415 * t2615 * t5594;
    let t19646 = 5.84605 * t19645;
    let t19647 = t5607 * t19583;
    let t19648 = 3.8973666666666666 * t19647;
    let t19650 = t415 * t2619 * t5594;
    let t19651 = 1.9486833333333333 * t19650;
    let t19653 = t1872 * t1832;
    (t19636, t19639, t19640, t19641, t19642, t19643, t19646, t19648, t19651, t19653)
}
