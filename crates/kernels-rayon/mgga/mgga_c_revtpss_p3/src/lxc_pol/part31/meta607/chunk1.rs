//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2046/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2046(t98187: f64, t5706: f64, t94429: f64, t1941: f64, t9817: f64, t5651: f64, t7028: f64, t9736: f64, t13985: f64, t94423: f64, t13878: f64, t25972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98188 = 0.50820002809285328226e-4_f64 * t98187;
    let t98193 = t94429 * t5706;
    let t98194 = 0.16006300097412701803e-1_f64 * t98193;
    let t98196 = t1941 * t9817;
    let t98200 = t9736 * t7028 * t5651;
    let t98202 = t94423 * t13985;
    let t98203 = 0.2032800112371413129e-3_f64 * t98202;
    let t98206 = t25972 * t13878;
    (t98188, t98194, t98196, t98200, t98203, t98206)
}
