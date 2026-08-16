//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 962/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk962<F: Float>(t1908: F, t5220: F, t1447: F, t6518: F, t1423: F, t6524: F, t6783: F, t1925: F, t5194: F, t6788: F, t5105: F, t831: F) -> (F, F, F, F, F, F, F) {
    let t15196 = t5220 * t1908;
    let t15208 = t1447 * t6518;
    let t15216 = t1423 * t6524;
    let t15237 = t1447 * t6783;
    let t15244 = t5194 * t1925;
    let t15248 = t1423 * t6788;
    let t15256 = t831 * t5105;
    (t15196, t15208, t15216, t15237, t15244, t15248, t15256)
}
