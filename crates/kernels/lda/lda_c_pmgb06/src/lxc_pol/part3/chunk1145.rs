//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1145/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1145<F: Float>(t12535: F, t495: F, t5065: F, t12539: F, t5069: F, t10269: F, t10273: F, t10286: F, t161: F, t1639: F, t166: F, t4935: F) -> (F, F, F, F, F) {
    let t13672 = t5065 * t12535 * t495;
    let t13675 = F::new(8.0) / F::new(15.0) * t13672 * t5069 * t12539;
    let t13676 = F::new(4.0) / F::new(45.0) * t10269;
    let t13677 = F::new(4.0) / F::new(45.0) * t10273;
    let t13678 = F::new(4.0) / F::new(45.0) * t10286;
    let t13682 = t161 * t166 * t1639 * t4935 / F::new(10.0);
    (t13675, t13676, t13677, t13678, t13682)
}
