//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 853/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk853<F: Float>(t1325: F, t3774: F, t3787: F, t3779: F, t519: F, t1476: F, t3742: F, t163: F, t169: F, t234: F, t2817: F, t4263: F, t466: F, t161: F, t8801: F, t148: F) -> (F, F, F, F, F, F, F) {
    let t10722 = t1325 * t3787 * t3774;
    let t10725 = t519 * t3787 * t3779;
    let t10729 = t3742 * t1476;
    let t10749 = 0.4097848972398244 * t169 * t2817 * t234 * t163;
    let t10750 = t466 * t4263;
    let t10752 = t8801 * t161;
    let t10755 = 0.031505407223141116 * t148 * t10752 * t163;
    (t10722, t10725, t10729, t10749, t10750, t10752, t10755)
}
