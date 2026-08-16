//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 721/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk721<F: Float>(t1342: F, t1249: F, t325: F, t128: F, t348: F, t107: F, t1248: F, t837: F, t899: F, t124: F, t507: F, t7190: F) -> (F, F, F, F, F, F, F) {
    let t26077 = t1342 * t1342;
    let t26078 = F::cast_from(1.0_f64) / t26077;
    let t26093 = t1249 * t325;
    let t26115 = t348 * t128;
    let t26125 = t1248 * t107;
    let t26144 = t899 * t837;
    let t26157 = t507 * t124;
    let t26283 = t507 * t7190;
    (t26078, t26093, t26115, t26125, t26144, t26157, t26283)
}
