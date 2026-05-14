//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1051/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1051<F: Float>(t13034: F, t13038: F, t13041: F, t13046: F, t13049: F, t13056: F, t13060: F, t13063: F, t13067: F, t13071: F, t13074: F, t10711: F, t13075: F, t13076: F, t13080: F, t13081: F, t13082: F, t13084: F, t13085: F, t13088: F, t13091: F, t13093: F, t13095: F) -> (F, F) {
    let t14405 = -t13034 - t13038 - t13041 + t13046 - t13049 - t13056 - t13060 - t13063 + t13067 + t13071 + t13074;
    let t14406 = -t13075 - t13076 - t13080 - t13081 + t13082 + t13084 - t13085 + t13088 - t13091 - t13093 - t13095 + t10711;
    (t14405, t14406)
}
