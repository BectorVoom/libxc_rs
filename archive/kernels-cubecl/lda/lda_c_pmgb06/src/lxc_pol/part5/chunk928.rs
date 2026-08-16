//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 928/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk928<F: Float>(t1830: F, t810: F, t1865: F, t947: F, t1860: F, t139: F, t30: F, t35: F) -> (F, F, F, F, F) {
    let t12337 = t1830 * t810;
    let t12366 = t947 * t1865;
    let t12368 = t947 * t1860;
    let t12369 = F::cast_from(0.0016792592592592592_f64) * t12368;
    let t12396 = t30 * t35 * t139;
    (t12337, t12366, t12368, t12369, t12396)
}
