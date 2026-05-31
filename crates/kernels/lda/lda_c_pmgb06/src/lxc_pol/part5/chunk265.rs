//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 265/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk265<F: Float>(t188: F, t438: F, t492: F, t542: F, t547: F, t549: F, t804: F, t808: F, t817: F, t826: F, t833: F, t837: F, t846: F, t855: F, t856: F) -> F {
    let t859 = t804 + t438 + t808 + t817 - t826 + t833 + t492 + t837 + t846 - t855 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t856 * t188 + t542 + t547 + t549;
    t859
}
