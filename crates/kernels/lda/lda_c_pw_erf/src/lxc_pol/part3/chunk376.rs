//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 376/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk376<F: Float>(t1351: F, t951: F, t1349: F, t11: F, t1333: F, t557: F, t558: F, t954: F, t1260: F, t190: F, t212: F, t331: F, t590: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1352 = t1351 * t951;
    let t1353 = t1349 * t1352;
    let t1354 = t11 * t1353;
    let t1356 = t1333 * t951;
    let t1357 = t557 * t1356;
    let t1358 = t11 * t1357;
    let t1360 = t558 * t954;
    let t1361 = t557 * t1360;
    let t1362 = t11 * t1361;
    let t1366 = F::new(0.011111111111111112) * t190 * t1260 * t212;
    let t1367 = t331 * t590;
    (t1352, t1353, t1354, t1356, t1357, t1358, t1360, t1361, t1362, t1366, t1367)
}
