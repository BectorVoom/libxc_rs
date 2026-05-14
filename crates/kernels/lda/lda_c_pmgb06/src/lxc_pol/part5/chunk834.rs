//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 834/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk834<F: Float>(t113: F, t11676: F, t301: F, t8131: F, t122: F, t4182: F, t886: F, t199: F, t5567: F, t1135: F, t868: F, t107: F, t1180: F, t2164: F, t2786: F, t902: F) -> (F, F, F, F, F, F, F) {
    let t11678 = t11676 * t113 * t301;
    let t11694 = 48.0 * t8131;
    let t11726 = t122 * t4182 * t886;
    let t11731 = t5567 * t199;
    let t11733 = t1135 * t868;
    let t11744 = t107 * t1180 * t2164;
    let t11745 = 3.9861630686838536 * t11744;
    let t11747 = t107 * t2786 * t902;
    (t11678, t11694, t11726, t11731, t11733, t11745, t11747)
}
