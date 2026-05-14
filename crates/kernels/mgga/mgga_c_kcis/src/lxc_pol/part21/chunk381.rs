//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 381/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk381<F: Float>(t2354: F, t2355: F, t680: F, t2318: F, t2321: F, t2323: F, t2327: F, t2329: F, t2331: F) -> (F, F) {
    let t2357 = t2354 * t2355 * t680;
    let t2366 = -0.57538888888888888889e0 * t2318 + 0.11507777777777777778e1 * t2321 + 0.40256666666666666667e0 * t2323 + 0.366775e-1 * t2327 + 0.73355e-1 * t2329 + 0.137975e0 * t2331;
    (t2357, t2366)
}
