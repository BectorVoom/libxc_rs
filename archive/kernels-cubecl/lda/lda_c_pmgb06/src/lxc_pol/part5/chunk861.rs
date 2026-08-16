//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 861/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk861<F: Float>(t109: F, t138: F, t3674: F, t3676: F, t1767: F, t282: F, t55: F, t691: F, t1062: F, t3709: F, t696: F, t957: F) -> (F, F, F) {
    let t8655 = F::cast_from(6.87343803774119_f64) * t138 * t109 * t3674 * t3676;
    let t8659 = F::cast_from(0.0018989649058080863_f64) * t691 * t55 * t1767 * t282;
    let t8668 = F::cast_from(623.3709278045327_f64) * t696 * t3709 * t957 * t1062;
    (t8655, t8659, t8668)
}
