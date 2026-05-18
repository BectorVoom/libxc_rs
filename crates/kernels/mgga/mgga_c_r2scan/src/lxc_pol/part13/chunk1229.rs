//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1229/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1229<F: Float>(t39347: F, t39351: F, t40271: F, t40278: F, t40284: F, t40288: F, t40292: F, t40300: F, t40302: F, t40327: F, t40329: F, t40338: F, t40348: F, t40351: F, t40353: F, t40360: F) -> F {
    let t40728 = -t39347 + t39351 + t40271 - t40278 - t40284 - t40288 + t40292 + t40300 - t40302 + t40327 + t40329 - t40338 - t40348 - t40351 + t40353 + t40360;
    t40728
}
