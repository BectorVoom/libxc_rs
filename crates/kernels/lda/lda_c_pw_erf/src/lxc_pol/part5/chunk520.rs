//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 520/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk520<F: Float>(t2758: F, t1035: F, t344: F, t137: F, t142: F) -> (F, F, F, F) {
    let t2759 = 0.10685 * t2758;
    let t2760 = t344 * t1035;
    let t2761 = 12.0 * t2760;
    let t2765 = t137 * t142;
    (t2759, t2760, t2761, t2765)
}
