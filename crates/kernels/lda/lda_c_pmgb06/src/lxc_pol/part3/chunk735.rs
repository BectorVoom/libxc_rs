//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 735/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk735<F: Float>(t4905: F, t4934: F, t518: F, t166: F, t161: F, t1639: F, t2088: F, t1586: F, t2093: F, t2094: F, t489: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4935 = t4905 + t4934;
    let t4936 = t518 * t4935;
    let t4937 = t166 * t4936;
    let t4939 = t161 * t4937 / F::cast_from(30.0_f64);
    let t4940 = t1639 * t2088;
    let t4941 = t166 * t4940;
    let t4943 = t161 * t4941 / F::cast_from(15.0_f64);
    let t4944 = t2093 * t1586;
    let t4945 = t166 * t4944;
    let t4947 = t161 * t4945 / F::cast_from(30.0_f64);
    let t4948 = t489 * t2094;
    let t4950 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t161 * t4948;
    (t4935, t4936, t4937, t4939, t4940, t4941, t4943, t4944, t4945, t4947, t4948, t4950)
}
