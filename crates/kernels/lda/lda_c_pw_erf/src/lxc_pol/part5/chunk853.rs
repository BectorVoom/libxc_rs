//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 853/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk853<F: Float>(t191: F, t21: F, t24: F, t1267: F, t3476: F, t3515: F, t3518: F, t10: F, t4: F, t56: F) -> (F, F, F, F) {
    let t11854 = t21 * t24 * t191;
    let t11855 = t1267 * t3476;
    let t11861 = t3515 * t3518;
    let t11866 = t4 * t10 * t56;
    (t11854, t11855, t11861, t11866)
}
