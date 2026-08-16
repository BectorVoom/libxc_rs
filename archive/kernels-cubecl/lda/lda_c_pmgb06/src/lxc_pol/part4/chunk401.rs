//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 401/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk401<F: Float>(t1455: F, t493: F, t1412: F, t1419: F, t1422: F, t1425: F, t1429: F, t1433: F, t1443: F, t1446: F, t1449: F, t1453: F) -> (F, F) {
    let t1457 = t493 * t1455 / F::cast_from(45.0_f64);
    let t1458 = t1412 - t1419 + t1422 + t1425 + t1429 + t1433 + t1443 + t1446 + t1449 + t1453 + t1457;
    (t1457, t1458)
}
