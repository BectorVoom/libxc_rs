//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1346/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1346<F: Float>(t13713: F, t2470: F, t3198: F, t13719: F, t13721: F, t13104: F, t835: F, t1977: F, t5305: F, t1847: F, t1980: F, t1983: F) -> (F, F, F, F, F, F, F) {
    let t17680 = F::cast_from(8.0_f64) / F::cast_from(405.0_f64) * t13713;
    let t17682 = t3198 * t2470 / F::cast_from(27.0_f64);
    let t17683 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13719;
    let t17684 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t13721;
    let t17686 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13104 * t835;
    let t17688 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t5305 * t1977;
    let t17691 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1847 * t1980 * t1983;
    (t17680, t17682, t17683, t17684, t17686, t17688, t17691)
}
