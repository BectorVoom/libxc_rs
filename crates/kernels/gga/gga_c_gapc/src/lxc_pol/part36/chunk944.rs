//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 944/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk944<F: Float>(t19120: F, t3765: F, t11730: F, t7553: F, t190: F, t4043: F, t19094: F, t19097: F, t291: F, t128: F, t188: F, t3707: F) -> (F, F, F, F, F) {
    let t33263 = t19120 * t3765;
    let t33265 = t7553 * t11730;
    let t33267 = t190 * t4043;
    let t33270 = t19094 * t33267 * t291 * t19097;
    let t33273 = t3707 * t188 * t128;
    (t33263, t33265, t33267, t33270, t33273)
}
