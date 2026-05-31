//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 975/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk975<F: Float>(t377: F, t5829: F, t1291: F, t1296: F, t1309: F, t2238: F, t2255: F, t3622: F, t3633: F, t3656: F, t384: F, t5831: F, t5843: F, t5880: F, t787: F, t8396: F, t8404: F, t8413: F) -> F {
    let t11535 = t5829 * t377;
    let t11558 = F::cast_from(6.0_f64) * t1296 * t2255 * t1309 + F::cast_from(2.0_f64) * t1296 * t787 * t3656 + F::cast_from(6.0_f64) * t1296 * t5880 * t384 + F::cast_from(24.0_f64) * t8413 * t787 * t3633 - F::cast_from(3.0_f64) * t11535 * t384 - F::cast_from(3.0_f64) * t1291 * t5880 - F::cast_from(3.0_f64) * t5831 * t1309 - t2238 * t3656 - F::cast_from(3.0_f64) * t3622 * t2255 - F::cast_from(18.0_f64) * t8404 * t5843 - t8396 * t787;
    t11558
}
