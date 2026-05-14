//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 815/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk815<F: Float>(t132: F, t3291: F, t435: F, t3295: F, t432: F, t1447: F, t3169: F, t1423: F, t3186: F, t1560: F, t3220: F, t3213: F, t3217: F, t3195: F, t1427: F, t1511: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9777 = t132 * t435 * t3291;
    let t9781 = t432 * t3295;
    let t9805 = t1447 * t3169;
    let t9821 = t1423 * t3186;
    let t9826 = t3220 * t1560;
    let t9828 = t3213 * t1560;
    let t9830 = t1423 * t3217;
    let t9832 = t1447 * t3195;
    let t9834 = t3220 * t1427;
    let t9836 = t1511 * t607;
    (t9777, t9781, t9805, t9821, t9826, t9828, t9830, t9832, t9834, t9836)
}
