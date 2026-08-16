//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 995/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk995<F: Float>(t161: F, t489: F, t6730: F, t132: F, t435: F, t6226: F, t1600: F, t6904: F, t2485: F, t3220: F, t1423: F, t6250: F) -> (F, F, F, F, F) {
    let t17938 = t161 * t489 * t6730;
    let t17960 = t132 * t435 * t6226;
    let t17964 = t1600 * t6904;
    let t17982 = t3220 * t2485;
    let t17984 = t1423 * t6250;
    (t17938, t17960, t17964, t17982, t17984)
}
