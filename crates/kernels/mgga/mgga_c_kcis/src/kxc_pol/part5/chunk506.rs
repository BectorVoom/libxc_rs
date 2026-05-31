//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 506/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk506<F: Float>(t2354: F, t2355: F, t680: F, t2318: F, t2321: F, t2323: F, t2327: F, t2329: F, t2331: F) -> (F, F) {
    let t2357 = t2354 * t2355 * t680;
    let t2366 = -F::cast_from(0.57538888888888888889e0_f64) * t2318 + F::cast_from(0.11507777777777777778e1_f64) * t2321 + F::cast_from(0.40256666666666666667e0_f64) * t2323 + F::cast_from(0.366775e-1_f64) * t2327 + F::cast_from(0.73355e-1_f64) * t2329 + F::cast_from(0.137975e0_f64) * t2331;
    (t2357, t2366)
}
