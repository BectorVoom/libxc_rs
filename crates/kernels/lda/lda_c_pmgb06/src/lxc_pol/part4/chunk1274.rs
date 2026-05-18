//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1274/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1274<F: Float>(t5: F, t1837: F, t1848: F, t5417: F, t831: F, t1542: F, t2592: F, t1074: F, t12429: F, t16322: F, t2381: F, t247: F, t332: F, t395: F, t5961: F, t6695: F, t760: F, zeta_threshold: F) -> (F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t16755 = F::new(2.0) / F::new(15.0) * t1848 * t1837;
    let t16757 = t831 * t5417 / F::new(15.0);
    let t16759 = t2592 * t1542 / F::new(30.0);
    let t16769 = piecewise3::<f64>(t6, F::new(0.0), F::new(2.0) * t1074 * t2381 - F::new(24.0) * t247 * t6695 + F::new(4.0) * t332 * t5961 + F::new(8.0) * t395 * t760 + t12429 + t16322);
    (t16755, t16757, t16759, t16769)
}
