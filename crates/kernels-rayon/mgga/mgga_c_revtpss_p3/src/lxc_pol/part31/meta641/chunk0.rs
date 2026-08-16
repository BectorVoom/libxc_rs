//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2098/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2098(t98285: f64, t98964: f64, t98976: f64, t98979: f64, t99009: f64, t99013: f64, t99035: f64, t99044: f64, t99050: f64, t99091: f64, t99113: f64, t30160: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t102569 = 0.72286371995927450867e-4_f64 * t98285;
    let t103264 = 0.30488190661738479625e-3_f64 * t98964;
    let t103269 = 0.72286371995927450867e-4_f64 * t98976;
    let t103270 = 0.10164000561857065645e-4_f64 * t98979;
    let t103285 = 0.90702367218671976884e-1_f64 * t99009;
    let t103287 = 0.2168320119862840671e-2_f64 * t99013;
    let t103297 = 0.22675591804667994221e-1_f64 * t99035;
    let t103302 = 0.40656002247428262579e-4_f64 * t99044;
    let t103305 = 35.0_f64 / 108.0_f64 * t99050;
    let t103329 = 0.1219527626469539185e-2_f64 * t99091;
    let t103347 = 0.18071592998981862717e-4_f64 * t99113;
    let t105814 = t30160 * t575;
    (t102569, t103264, t103269, t103270, t103285, t103287, t103297, t103302, t103305, t103329, t103347, t105814)
}
