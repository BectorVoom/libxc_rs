//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1151/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1151<F: Float>(t2002: F, t3210: F, t1963: F, t3220: F, t1423: F, t4780: F, t4615: F, t1420: F, t4609: F, t1969: F, t3177: F, t1447: F, t5337: F) -> (F, F, F, F, F, F, F, F) {
    let t13739 = t2002 * t3210 / F::cast_from(5.0_f64);
    let t13740 = t3220 * t1963;
    let t13741 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13740;
    let t13742 = t1423 * t4780;
    let t13743 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t13742;
    let t13744 = t1423 * t4615;
    let t13745 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13744;
    let t13747 = t1420 * t4609 / F::cast_from(5.0_f64);
    let t13748 = t3220 * t1969;
    let t13749 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t13748;
    let t13751 = t3177 * t1969 / F::cast_from(5.0_f64);
    let t13752 = t1447 * t5337;
    (t13739, t13741, t13743, t13745, t13747, t13749, t13751, t13752)
}
