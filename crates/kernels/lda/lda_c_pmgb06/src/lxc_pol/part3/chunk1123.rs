//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1123/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1123<F: Float>(t10158: F, t10161: F, t11991: F, t1476: F, t36: F, t350: F, t4862: F, t12864: F, t506: F, t4641: F, t4867: F, t12563: F, t2909: F) -> (F, F, F, F, F, F, F) {
    let t13327 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10158;
    let t13328 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10161;
    let t13330 = t36 * t1476 * t11991;
    let t13332 = t350 * t4862;
    let t13335 = t36 * t506 * t12864;
    let t13337 = t4641 * t4867;
    let t13340 = t36 * t2909 * t12563;
    (t13327, t13328, t13330, t13332, t13335, t13337, t13340)
}
