//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2011/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2011(t98964: f64, t98968: f64, t98972: f64, t98976: f64, t98979: f64, t92963: f64, t92966: f64, t92969: f64, t92971: f64, t92979: f64, t95666: f64, t98970: f64) -> f64 {
    let t103264 = 0.30488190661738479625e-3_f64 * t98964;
    let t103265 = 0.11433071498151929859e-2_f64 * t98968;
    let t103267 = 0.4065600224742826258e-3_f64 * t98972;
    let t103269 = 0.72286371995927450867e-4_f64 * t98976;
    let t103270 = 0.10164000561857065645e-4_f64 * t98979;
    let t103271 = 0.2032800112371413129e-4_f64 * t92963 - 0.14457274399185490174e-3_f64 * t92966 - 35.0_f64 / 54.0_f64 * t92969 + 7.0_f64 / 72.0_f64 * t92971 - t103264 - t103265 - 0.34299214494455789578e-2_f64 * t98970 - t103267 + t95666 - 7.0_f64 / 24.0_f64 * t92979 - t103269 + t103270;
    t103271
}
