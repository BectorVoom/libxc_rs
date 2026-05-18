//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 828/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk828<F: Float>(t1553: F, t776: F, t405: F, t247: F, t4713: F, t251: F, t2252: F, t652: F, t256: F, t19: F, t1904: F, t644: F) -> (F, F, F, F, F, F, F, F) {
    let t5782 = t776 * t1553;
    let t5783 = t405 * t5782;
    let t5787 = t4713 * t247;
    let t5788 = t5787 * t251;
    let t5791 = t2252 * t652;
    let t5793 = F::new(2.0) / F::new(3.0) * t5791 * t256;
    let t5794 = t1904 * t19;
    let t5795 = t5794 * t644;
    (t5782, t5783, t5787, t5788, t5791, t5793, t5794, t5795)
}
