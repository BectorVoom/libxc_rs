//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 821/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk821<F: Float>(t6617: F, t6619: F, t6622: F, t6624: F, t3311: F, t3324: F, t3327: F, t3331: F, t3335: F, t5675: F, t7750: F, t7752: F, t7754: F, t7756: F, t7758: F, t7759: F) -> (F, F, F, F, F) {
    let t7760 = t6617 / F::cast_from(15.0_f64);
    let t7761 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t6619;
    let t7762 = t6622 / F::cast_from(15.0_f64);
    let t7763 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t6624;
    let t7764 = -t7750 - t7752 - t7754 - t7756 + F::cast_from(8.0_f64) * t5675 - t3311 + t3324 + t3327 + t3331 - t3335 - t7758 - t7759 - t7760 - t7761 - t7762 - t7763;
    (t7760, t7761, t7762, t7763, t7764)
}
