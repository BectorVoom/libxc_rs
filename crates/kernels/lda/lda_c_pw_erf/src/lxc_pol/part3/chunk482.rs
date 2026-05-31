//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 482/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk482<F: Float>(t611: F, t838: F, t1387: F, t1398: F, t515: F, t795: F, t325: F, t817: F, t1351: F, t743: F, t352: F) -> (F, F, F, F, F, F, F) {
    let t1934 = t838 * t611;
    let t1936 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1387;
    let t1937 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1398;
    let t1938 = t795 * t515;
    let t1939 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1938;
    let t1941 = t325 * t817;
    let t1943 = t1351 * t743;
    let t1944 = t1943 * t352;
    (t1934, t1936, t1937, t1939, t1941, t1943, t1944)
}
