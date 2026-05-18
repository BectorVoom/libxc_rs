//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 904/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk904<F: Float>(t1423: F, t3217: F, t1447: F, t3195: F, t1427: F, t3220: F, t1511: F, t607: F, t446: F, t3012: F, t3204: F, t161: F, t3446: F, t489: F) -> (F, F, F, F, F, F, F, F) {
    let t9830 = t1423 * t3217;
    let t9832 = t1447 * t3195;
    let t9834 = t3220 * t1427;
    let t9836 = t1511 * t607;
    let t9837 = t9836 * t446;
    let t9847 = t1423 * t3012;
    let t9853 = t1447 * t3204;
    let t9885 = t161 * t489 * t3446;
    (t9830, t9832, t9834, t9836, t9837, t9847, t9853, t9885)
}
