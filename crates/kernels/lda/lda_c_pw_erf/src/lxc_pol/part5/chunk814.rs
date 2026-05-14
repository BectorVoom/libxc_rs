//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 814/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk814<F: Float>(t159: F, t285: F, t8756: F, t2863: F, t684: F, t2874: F, t8520: F, t281: F, t2872: F, t465: F, t2824: F, t6: F, t153: F, t343: F, t4606: F, t5021: F) -> (F, F, F, F, F, F, F) {
    let t8759 = 0.03831185177913979 * t8756 * t159 * t285;
    let t8771 = t684 * t2863;
    let t8774 = 0.07982822673503073 * t684 * t2874;
    let t8777 = 120.0 * t8520;
    let t8793 = t281 * t465 * t2872 * t285;
    let t8798 = t6 * t2824;
    let t8801 = 0.017888888888888888 * t4606 + 0.22252592592592593 * t5021 - 0.07316671043820612 * t343 + 0.015663796296296297 * t153 * t8798;
    (t8759, t8771, t8774, t8777, t8793, t8798, t8801)
}
