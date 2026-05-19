//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 585/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk585<F: Float>(t1052: F, t3230: F, t3233: F, t1059: F, t1067: F, t3332: F, t3339: F, t3330: F, t119: F, t2983: F, t1041: F, t1025: F, t1062: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4386 = t1052 * t3230;
    let t4388 = t1052 * t3233;
    let t4391 = t1059 * t1067;
    let t4397 = F::cast_from(0.13650364140255672_f64) * t3332;
    let t4398 = F::cast_from(0.02275060690042612_f64) * t3339;
    let t4406 = F::cast_from(0.10237773105191754_f64) * t3330;
    let t4411 = t119 * t2983;
    let t4413 = t1041 * t1067;
    let t4420 = t1025 * t1062;
    (t4386, t4388, t4391, t4397, t4398, t4406, t4411, t4413, t4420)
}
