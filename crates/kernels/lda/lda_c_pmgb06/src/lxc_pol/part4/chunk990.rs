//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 990/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk990<F: Float>(t13726: F, t446: F, t1427: F, t5220: F, t1431: F, t1441: F, t1963: F, t3220: F, t1423: F, t4780: F, t4615: F, t1969: F, t1447: F, t5337: F, t5477: F, t5268: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13727 = t13726 * t446;
    let t13729 = t5220 * t1427;
    let t13731 = t5220 * t1431;
    let t13733 = t5220 * t1441;
    let t13740 = t3220 * t1963;
    let t13742 = t1423 * t4780;
    let t13744 = t1423 * t4615;
    let t13748 = t3220 * t1969;
    let t13752 = t1447 * t5337;
    let t13756 = t1447 * t5477;
    let t13758 = t1423 * t5268;
    (t13727, t13729, t13731, t13733, t13740, t13742, t13744, t13748, t13752, t13756, t13758)
}
