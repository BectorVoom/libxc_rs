//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2046/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2046<F: Float>(t98187: F, t5706: F, t94429: F, t1941: F, t9817: F, t5651: F, t7028: F, t9736: F, t13985: F, t94423: F, t13878: F, t25972: F) -> (F, F, F, F, F, F) {
    let t98188 = F::cast_from(0.50820002809285328226e-4_f64) * t98187;
    let t98193 = t94429 * t5706;
    let t98194 = F::cast_from(0.16006300097412701803e-1_f64) * t98193;
    let t98196 = t1941 * t9817;
    let t98200 = t9736 * t7028 * t5651;
    let t98202 = t94423 * t13985;
    let t98203 = F::cast_from(0.2032800112371413129e-3_f64) * t98202;
    let t98206 = t25972 * t13878;
    (t98188, t98194, t98196, t98200, t98203, t98206)
}
