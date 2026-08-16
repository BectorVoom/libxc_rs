//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2102/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2102(t5651: f64, t7028: f64, t9736: f64, t13985: f64, t94423: f64, t13869: f64, t7271: f64, t13878: f64, t25972: f64, t94468: f64, t94472: f64, t94474: f64, t98186: f64, t98188: f64, t98189: f64, t98191: f64, t98194: f64, t98197: f64) -> f64 {
    let t98200 = t9736 * t7028 * t5651;
    let t98202 = t94423 * t13985;
    let t98203 = 0.2032800112371413129e-3_f64 * t98202;
    let t98204 = t7271 * t13869;
    let t98206 = t25972 * t13878;
    let t98207 = 0.10164000561857065645e-2_f64 * t98206;
    let t98208 = t98186 - t98188 - 0.34299214494455789578e-2_f64 * t98189 + 0.17149607247227894789e-2_f64 * t98191 - t98194 - 0.25410001404642664113e-4_f64 * t94468 - t98197 / 4.0_f64 - t94472 + 0.2032800112371413129e-4_f64 * t98200 + t94474 + t98203 + 0.17149607247227894789e-1_f64 * t98204 + t98207;
    t98208
}
