//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 409/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk409<F: Float>(t5: F, t161: F, t1933: F, t490: F, t831: F, t1393: F, t607: F, t883: F, t10: F, t760: F, t1: F, t594: F, t332: F, t395: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t1934 = t161 * t1933;
    let t1935 = t1934 / F::new(45.0);
    let t1936 = t831 * t490;
    let t1937 = t1936 / F::new(45.0);
    let t1938 = t1393 / F::new(45.0);
    let t1939 = t883 * t607;
    let t1941 = t10 * t760;
    let t1944 = t594 * t1;
    let t1948 = piecewise3::<F>(t6, F::new(0.0), F::new(40.0) / F::new(9.0) * t1941 * t332 + F::new(16.0) / F::new(3.0) * t1944 * t395);
    (t1934, t1935, t1936, t1937, t1938, t1939, t1941, t1948)
}
