//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1057/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1057<F: Float>(t2615: F, t415: F, t5594: F, t19583: F, t5607: F, t2619: F, t443: F, t7166: F, t1710: F, t2630: F, t1870: F, t5639: F, t7191: F) -> (F, F, F, F, F, F) {
    let t19645 = t415 * t2615 * t5594;
    let t19647 = t5607 * t19583;
    let t19650 = t415 * t2619 * t5594;
    let t19703 = t7166 * t443;
    let t19726 = t2630 * t1710;
    let t19739 = t1870 * t5639 * t7191;
    (t19645, t19647, t19650, t19703, t19726, t19739)
}
