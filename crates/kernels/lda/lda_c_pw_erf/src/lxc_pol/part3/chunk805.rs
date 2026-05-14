//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 805/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk805<F: Float>(t159: F, t285: F, t8756: F, t142: F, t3363: F, t454: F, t1553: F, t1726: F, t405: F, t2863: F, t684: F, t2874: F, t8520: F, t281: F, t2853: F, t477: F) -> (F, F, F, F, F, F, F) {
    let t8759 = 0.03831185177913979 * t8756 * t159 * t285;
    let t8761 = t454 * t3363 * t142;
    let t8768 = t405 * t1726 * t1553;
    let t8771 = t684 * t2863;
    let t8774 = 0.07982822673503073 * t684 * t2874;
    let t8777 = 120.0 * t8520;
    let t8785 = t281 * t2853 * t477 * t285;
    (t8759, t8761, t8768, t8771, t8774, t8777, t8785)
}
