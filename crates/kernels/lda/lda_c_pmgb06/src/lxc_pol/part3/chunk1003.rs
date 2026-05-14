//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1003/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1003<F: Float>(t2002: F, t3210: F, t1963: F, t3220: F, t1423: F, t4780: F, t4615: F, t1420: F, t4609: F, t1969: F, t3177: F, t1447: F, t5337: F, t1972: F, t2873: F, t5477: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13739 = t2002 * t3210 / 5.0;
    let t13740 = t3220 * t1963;
    let t13741 = 4.0 / 45.0 * t13740;
    let t13742 = t1423 * t4780;
    let t13743 = 4.0 / 45.0 * t13742;
    let t13744 = t1423 * t4615;
    let t13745 = 2.0 / 45.0 * t13744;
    let t13747 = t1420 * t4609 / 5.0;
    let t13748 = t3220 * t1969;
    let t13749 = 4.0 / 15.0 * t13748;
    let t13751 = t3177 * t1969 / 5.0;
    let t13752 = t1447 * t5337;
    let t13753 = 4.0 / 45.0 * t13752;
    let t13755 = 2.0 / 15.0 * t1972 * t2873;
    let t13756 = t1447 * t5477;
    (t13739, t13741, t13743, t13745, t13747, t13749, t13751, t13753, t13755, t13756)
}
