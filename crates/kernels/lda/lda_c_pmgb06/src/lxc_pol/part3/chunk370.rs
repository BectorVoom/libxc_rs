//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 370/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk370<F: Float>(t1395: F, t477: F, t137: F, t132: F, t1166: F, t183: F, t398: F, t539: F, t188: F, t1368: F, t1370: F, t1374: F, t1379: F, t1384: F, t1389: F, t1391: F, t1394: F) -> (F, F, F, F, F, F, F) {
    let t1396 = t1395 * t477;
    let t1397 = t137 * t1396;
    let t1399 = t132 * t1397 / 15.0;
    let t1400 = t1166 * t183;
    let t1403 = t398 * t539;
    let t1404 = t1403 * t188;
    let t1406 = t1368 + 0.21642082724729686 * t1370 + t1374 + t1379 - t1384 - t1389 - t1391 - t1394 - t1399 + 4.0 / 3.0 * t1400 * t188 + 8.0 / 3.0 * t1404;
    (t1396, t1397, t1399, t1400, t1403, t1404, t1406)
}
