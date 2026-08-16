//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 228/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk228<F: Float>(t1305: F, t1328: F, t1309: F, t1320: F, t1325: F, t1332: F) -> (F, F, F) {
    let t1349 = F::cast_from(0.301925e0_f64) * t1305;
    let t1352 = F::cast_from(0.82785e-1_f64) * t1328;
    let t1354 = F::cast_from(0.258925e1_f64) * t1320 - t1349 - F::cast_from(0.301925e0_f64) * t1309 + F::cast_from(0.16504875e0_f64) * t1325 - t1352 - F::cast_from(0.82785e-1_f64) * t1332;
    (t1349, t1352, t1354)
}
