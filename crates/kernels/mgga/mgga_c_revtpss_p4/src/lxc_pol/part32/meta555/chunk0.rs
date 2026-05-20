//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1874/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1874<F: Float>(t5665: F, t94497: F, t14036: F, t25997: F, t13941: F, t94423: F, t14005: F, t5706: F, t94429: F, t1941: F, t9817: F, t5651: F, t7028: F, t9736: F) -> (F, F, F, F, F, F, F) {
    let t98174 = t94497 * t5665;
    let t98180 = t25997 * t14036;
    let t98185 = t94423 * t13941;
    let t98187 = t94423 * t14005;
    let t98193 = t94429 * t5706;
    let t98196 = t1941 * t9817;
    let t98200 = t9736 * t7028 * t5651;
    (t98174, t98180, t98185, t98187, t98193, t98196, t98200)
}
