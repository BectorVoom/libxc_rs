//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2114/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2114<F: Float>(t5651: F, t7028: F, t9736: F, t13985: F, t94423: F, t13869: F, t7271: F, t13878: F, t25972: F, t94468: F, t94472: F, t94474: F, t98186: F, t98188: F, t98189: F, t98191: F, t98194: F, t98197: F) -> F {
    let t98200 = t9736 * t7028 * t5651;
    let t98202 = t94423 * t13985;
    let t98203 = F::cast_from(0.2032800112371413129e-3_f64) * t98202;
    let t98204 = t7271 * t13869;
    let t98206 = t25972 * t13878;
    let t98207 = F::cast_from(0.10164000561857065645e-2_f64) * t98206;
    let t98208 = t98186 - t98188 - F::cast_from(0.34299214494455789578e-2_f64) * t98189 + F::cast_from(0.17149607247227894789e-2_f64) * t98191 - t98194 - F::cast_from(0.25410001404642664113e-4_f64) * t94468 - t98197 / F::cast_from(4.0_f64) - t94472 + F::cast_from(0.2032800112371413129e-4_f64) * t98200 + t94474 + t98203 + F::cast_from(0.17149607247227894789e-1_f64) * t98204 + t98207;
    t98208
}
