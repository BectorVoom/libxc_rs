//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 598/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk598<F: Float>(t396: F, t4767: F, t1494: F, t1653: F, t1493: F, t296: F, t343: F, t366: F, t1191: F, t1142: F, t13: F, t1147: F, t229: F) -> (F, F, F, F, F, F, F) {
    let t4769 = F::cast_from(0.9840332968370255_f64) * t396 * t4767;
    let t4770 = t1494 * t1653;
    let t4774 = F::new(1.0) / t1493 / t296;
    let t4775 = t4774 * t343;
    let t4782 = F::cast_from(2.1943705410881575_f64) * t366 * t4767;
    let t4785 = t1191 * t1191;
    let t4787 = t13 * t1142;
    let t4789 = t229 * t1147;
    (t4769, t4770, t4775, t4782, t4785, t4787, t4789)
}
