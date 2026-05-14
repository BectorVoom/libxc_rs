//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 883/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk883<F: Float>(t2620: F, t955: F, t405: F, t6879: F, t350: F, t6828: F, t2546: F, t947: F, t2542: F, t2550: F, t6885: F, t4913: F, t6888: F, t4641: F, t6816: F, t6805: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17035 = t955 * t2620;
    let t17041 = t405 * t6879;
    let t17054 = t350 * t6828;
    let t17059 = t947 * t2546;
    let t17061 = t947 * t2542;
    let t17066 = t947 * t2550;
    let t17127 = t405 * t6885;
    let t17129 = t4913 * t6888;
    let t17131 = t4641 * t6816;
    let t17133 = t350 * t6805;
    (t17035, t17041, t17054, t17059, t17061, t17066, t17127, t17129, t17131, t17133)
}
