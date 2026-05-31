//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1194/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1194<F: Float>(t1291: F, t1296: F, t21568: F, t21583: F, t21590: F, t21595: F, t2238: F, t2255: F, t2722: F, t2730: F, t3625: F, t3632: F, t378: F, t384: F, t7086: F, t7334: F, t7337: F, t7351: F, t787: F, t8404: F, t8413: F) -> F {
    let t21599 = -F::cast_from(3.0_f64) * t2238 * t7086 - F::cast_from(6.0_f64) * t8404 * t7334 + F::cast_from(24.0_f64) * t8413 * t7334 * t384 - F::cast_from(18.0_f64) * t3632 * t2722 * t2255 + F::cast_from(6.0_f64) * t3625 * t7337 - F::cast_from(18.0_f64) * t3632 * t7337 * t384 + F::cast_from(6.0_f64) * t1296 * t2255 * t2730 + F::cast_from(6.0_f64) * t1296 * t787 * t7086 - t1291 * t7351 + F::cast_from(2.0_f64) * t1296 * t7351 * t384 - t378 * (t21568 + t21583 + t21590 + t21595);
    t21599
}
