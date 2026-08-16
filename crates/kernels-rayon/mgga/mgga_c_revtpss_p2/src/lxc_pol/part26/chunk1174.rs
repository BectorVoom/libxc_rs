//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1174/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1174(t92988: f64, t92995: f64, t92997: f64, t92999: f64, t93007: f64, t93012: f64, t92979: f64, t92982: f64, t92984: f64, t92991: f64, t93001: f64, t93004: f64, t93010: f64, t93016: f64) -> f64 {
    let t95671 = 0.3252886739816735289e-3_f64 * t92988;
    let t95673 = 455.0_f64 / 648.0_f64 * t92995;
    let t95674 = 0.15117061203111996147e0_f64 * t92997;
    let t95675 = 0.51384669507166276316e-2_f64 * t92999;
    let t95678 = 0.80328230880474379779e-6_f64 * t93007;
    let t95680 = 0.45178982497454656792e-6_f64 * t93012;
    let t95682 = -7.0_f64 / 8.0_f64 * t92979 - t92982 / 2.0_f64 + 3.0_f64 / 8.0_f64 * t92984 - t95671 + 0.12196800674228478774e-3_f64 * t92991 - t95673 - t95674 + t95675 - 0.3658582879408617555e-2_f64 * t93001 + 0.34299214494455789577e-3_f64 * t93004 + t95678 - 0.17149607247227894789e-2_f64 * t93010 - t95680 - 0.54214778996945588151e-4_f64 * t93016;
    t95682
}
