//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 328/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk328<F: Float>(t1139: F, t1204: F, t1278: F, t1282: F, t1291: F, t187: F, t437: F, t828: F, t89: F) -> (F, F) {
    let t1295 = t1139 - t1204 + t187 * (t1278 * t437 - t1282 * t1291 - t1139 + t1204);
    let t1646 = -t89 - t828;
    (t1295, t1646)
}
