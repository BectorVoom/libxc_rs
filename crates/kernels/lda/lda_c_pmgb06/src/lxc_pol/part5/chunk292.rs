//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 292/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk292<F: Float>(t1011: F, t993: F, t242: F, t30: F, t410: F, t109: F, t621: F, t138: F, t634: F, t238: F, t620: F, t232: F) -> (F, F, F, F, F, F, F) {
    let t1012 = t993 * t1011;
    let t1017 = F::cast_from(0.0014764627977777779_f64) * t30 * t410 * t242;
    let t1018 = t109 * t621;
    let t1021 = F::cast_from(0.035616666666666665_f64) * t138 * t1018 * t634;
    let t1022 = t620 * t238;
    let t1023 = F::cast_from(1.0_f64) / t1022;
    let t1024 = t232 * t1023;
    (t1012, t1017, t1018, t1021, t1022, t1023, t1024)
}
