//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 827/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk827<F: Float>(t7832: F, t7856: F, t518: F, t166: F, t161: F, t2106: F, t2648: F, t137: F, t132: F, t3391: F, t3395: F, t7765: F, t7766: F, t7805: F, t7810: F, t7815: F, t7816: F, t7817: F, t7818: F, t7819: F, t7820: F, t7821: F, t7822: F) -> (F, F, F, F, F, F, F, F) {
    let t7857 = t7832 + t7856;
    let t7858 = t518 * t7857;
    let t7859 = t166 * t7858;
    let t7861 = t161 * t7859 / F::cast_from(30.0_f64);
    let t7862 = t2106 * t2648;
    let t7863 = t137 * t7862;
    let t7865 = t132 * t7863 / F::cast_from(10.0_f64);
    let t7866 = t7765 + t7766 - t7805 - t7810 - t7815 + t3391 + t3395 + t7816 + t7817 + t7818 + t7819 + t7820 - t7821 + t7822 - t7861 - t7865;
    (t7857, t7858, t7859, t7861, t7862, t7863, t7865, t7866)
}
