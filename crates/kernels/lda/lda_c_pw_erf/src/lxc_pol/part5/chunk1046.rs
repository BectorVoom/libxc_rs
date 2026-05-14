//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1046/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1046<F: Float>(t17788: F, t184: F, t2423: F, t549: F, t813: F, t17794: F, t352: F, t7414: F, t11: F, t557: F, t1953: F, t21219: F, t21231: F, t21235: F, t325: F, t7434: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21771 = 16.0 / 15.0 * t17788;
    let t21775 = 4.0 / 5.0 * t549 * t2423 * t184 * t813;
    let t21776 = 8.0 / 45.0 * t17794;
    let t21777 = t7414 * t352;
    let t21779 = t11 * t557 * t21777;
    let t21782 = t1953 * t557 * t21219;
    let t21785 = t11 * t557 * t21231;
    let t21788 = t1953 * t557 * t21235;
    let t21790 = t325 * t7434;
    (t21771, t21775, t21776, t21777, t21779, t21782, t21785, t21788, t21790)
}
