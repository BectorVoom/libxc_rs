//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 718/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk718<F: Float>(t5996: F, t6035: F, t7106: F, t7243: F, t6974: F, t6977: F, t2448: F, t38: F, t776: F, t5788: F, t5803: F, t2229: F, t5791: F, t5797: F, t5813: F, t63: F, t6968: F, t6971: F, t6984: F, t6987: F) -> (F, F, F, F, F, F, F) {
    let t7245 = t5996 + t6035 + t7106 + t7243;
    let t7261 = 2.923025 * t6974;
    let t7262 = 1.4615125 * t6977;
    let t7270 = 17.53815 * t38 * t776 * t2448;
    let t7271 = 1.9486833333333333 * t5788;
    let t7274 = 0.9743416666666667 * t5803;
    let t7276 = -8.81424 * t6968 + 2.20356 * t6971 - t7261 + t7262 - 3.0 / 2.0 * t6984 + t6987 / 2.0 + 17.62848 * t63 * t2229 * t2448 + t7270 - t7271 - 2.0 / 3.0 * t5791 - 1.46904 * t5797 + t7274 - 2.93808 * t5813;
    (t7245, t7261, t7262, t7270, t7271, t7274, t7276)
}
