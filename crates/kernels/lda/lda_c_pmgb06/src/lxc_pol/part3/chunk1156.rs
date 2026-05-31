//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1156/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1156<F: Float>(t1920: F, t3198: F, t1444: F, t5448: F, t5464: F, t10152: F, t493: F, t5336: F, t1447: F, t5333: F, t4602: F, t5458: F) -> (F, F, F, F, F, F) {
    let t13799 = t3198 * t1920 / F::cast_from(9.0_f64);
    let t13801 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1444 * t5448;
    let t13803 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1444 * t5464;
    let t13806 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t493 * t10152 * t5336;
    let t13807 = t1447 * t5333;
    let t13808 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t13807;
    let t13810 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4602 * t5458;
    (t13799, t13801, t13803, t13806, t13808, t13810)
}
