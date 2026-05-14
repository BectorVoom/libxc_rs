//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 909/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk909<F: Float>(t3787: F, t519: F, t6908: F, t1325: F, t6945: F, t6904: F, t2328: F, t3966: F, t2466: F, t4507: F, t10011: F, t6771: F, t10030: F, t6756: F, t2337: F, t352: F) -> (F, F, F, F, F, F, F, F) {
    let t17579 = t519 * t3787 * t6908;
    let t17591 = t1325 * t3787 * t6945;
    let t17594 = t519 * t3787 * t6904;
    let t17637 = t3966 * t2328;
    let t17645 = t4507 * t2466;
    let t17657 = t10011 * t6771;
    let t17664 = t10030 * t6756;
    let t17673 = t2337 * t352;
    (t17579, t17591, t17594, t17637, t17645, t17657, t17664, t17673)
}
