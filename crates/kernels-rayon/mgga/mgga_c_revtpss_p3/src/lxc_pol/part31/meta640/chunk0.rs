//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2097/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2097(t2045: f64, t5789: f64, t101451: f64, t98141: f64, t98148: f64, t98161: f64, t98165: f64, t98200: f64, t98218: f64, t98220: f64, t98224: f64, t98260: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t101674 = 2.0_f64 * t5789 * t2045;
    let t101754 = 22.0_f64 / 9.0_f64 * t101451;
    let t102486 = 0.30488190661738479625e-3_f64 * t98141;
    let t102489 = 0.2168320119862840671e-2_f64 * t98148;
    let t102495 = 0.10164000561857065645e-4_f64 * t98161;
    let t102498 = 0.90702367218671976884e-1_f64 * t98165;
    let t102515 = 0.40656002247428262579e-4_f64 * t98200;
    let t102526 = 0.1219527626469539185e-2_f64 * t98218;
    let t102527 = 0.18071592998981862717e-4_f64 * t98220;
    let t102529 = 0.22675591804667994221e-1_f64 * t98224;
    let t102549 = 35.0_f64 / 108.0_f64 * t98260;
    (t101674, t101754, t102486, t102489, t102495, t102498, t102515, t102526, t102527, t102529, t102549)
}
