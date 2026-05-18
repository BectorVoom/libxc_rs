//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1125/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1125<F: Float>(t12588: F, t1476: F, t1830: F, t12599: F, t2909: F, t12605: F, t36: F, t506: F, t12558: F, t1827: F, t947: F, t1822: F) -> (F, F, F, F, F, F) {
    let t13359 = t1830 * t1476 * t12588;
    let t13362 = t1830 * t2909 * t12599;
    let t13365 = t36 * t506 * t12605;
    let t13368 = t1830 * t506 * t12558;
    let t13370 = t947 * t1827;
    let t13372 = t947 * t1822;
    (t13359, t13362, t13365, t13368, t13370, t13372)
}
