//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 356/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk356<F: Float>(t1305: F, t1328: F, t1309: F, t1320: F, t1325: F, t1332: F) -> (F, F, F) {
    let t1349 = F::new(0.301925e0) * t1305;
    let t1352 = F::new(0.82785e-1) * t1328;
    let t1354 = F::new(0.258925e1) * t1320 - t1349 - F::new(0.301925e0) * t1309 + F::new(0.16504875e0) * t1325 - t1352 - F::new(0.82785e-1) * t1332;
    (t1349, t1352, t1354)
}
