//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 268/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk268<F: Float>(t5: F, t12: F, t208: F, t871: F, t594: F, t760: F, t598: F, t764: F, t44: F, t213: F, t224: F, t438: F, t492: F, t583: F, t590: F, t593: F, t609: F, t804: F, t808: F, t817: F, t826: F, t833: F, t837: F, t846: F, t855: F, zeta_threshold: F) -> (F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t872 = t871 * t208;
    let t877 = piecewise3::<F>(t6, F::new(0.0), F::new(8.0) / F::new(3.0) * t594 * t760);
    let t880 = piecewise3::<F>(t13, F::new(0.0), F::new(8.0) / F::new(3.0) * t598 * t764);
    let t883 = (t877 / F::new(2.0) + t880 / F::new(2.0)) * t44;
    let t886 = t804 + t438 + t808 + t817 - t826 + t833 + t492 + t837 + t846 - t855 + t872 * t213 / F::new(3.0) + t583 + t590 + t593 - t883 * t224 / F::new(15.0) - t609;
    (t872, t883, t886)
}
