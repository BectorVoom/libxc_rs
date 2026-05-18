//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 885/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk885<F: Float>(t153: F, t343: F, t4606: F, t5021: F, t8798: F, t147: F, t281: F, t285: F, t1138: F, t2817: F, t2820: F, t465: F) -> (F, F, F) {
    let t8801 = F::new(0.017888888888888888) * t4606 + F::new(0.22252592592592593) * t5021 - F::new(0.07316671043820612) * t343 + F::new(0.015663796296296297) * t153 * t8798;
    let t8805 = F::new(0.01197423401025461) * t281 * t147 * t8801 * t285;
    let t8808 = t2817 * t465 * t1138 * t2820;
    (t8801, t8805, t8808)
}
