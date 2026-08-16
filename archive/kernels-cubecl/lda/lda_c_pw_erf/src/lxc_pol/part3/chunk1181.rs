//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1181/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1181<F: Float>(t3416: F, t4943: F, t1627: F, t4537: F, t1926: F, t4204: F, t4183: F, t185: F, t4567: F, t4723: F, t1298: F, t4564: F) -> (F, F, F, F, F, F) {
    let t13914 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3416 * t4943;
    let t13915 = t4537 * t1627;
    let t13916 = F::cast_from(0.21642082724729686_f64) * t13915;
    let t13917 = t1926 * t4204;
    let t13919 = t1926 * t4183;
    let t13922 = t185 * t4567 * t4723;
    let t13923 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t13922;
    let t13924 = t1298 * t4564;
    (t13914, t13916, t13917, t13919, t13923, t13924)
}
