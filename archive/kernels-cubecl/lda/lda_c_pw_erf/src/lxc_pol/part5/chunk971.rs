//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 971/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk971<F: Float>(t3604: F, t4521: F, t3589: F, t4048: F, t581: F, t11753: F, t1627: F, t4537: F, t1926: F, t4204: F, t4183: F, t1298: F, t4564: F) -> (F, F, F, F, F, F, F) {
    let t13812 = t4521 * t3604;
    let t13829 = t4048 * t581 * t3589;
    let t13846 = F::cast_from(0.0016792592592592592_f64) * t11753;
    let t13915 = t4537 * t1627;
    let t13916 = F::cast_from(0.21642082724729686_f64) * t13915;
    let t13917 = t1926 * t4204;
    let t13919 = t1926 * t4183;
    let t13924 = t1298 * t4564;
    (t13812, t13829, t13846, t13916, t13917, t13919, t13924)
}
