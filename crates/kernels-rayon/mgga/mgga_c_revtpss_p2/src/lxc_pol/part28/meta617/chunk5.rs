//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2163/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2163(t14769: f64, t7045: f64, t14727: f64, t25227: f64, t2661: f64, t4430: f64, t93034: f64, t92991: f64, t14861: f64, t92989: f64, t98984: f64, t98985: f64, t98989: f64, t98992: f64, t98993: f64, t98995: f64) -> f64 {
    let t98997 = t7045 * t14769;
    let t99000 = t2661 * t25227 * t14727;
    let t99001 = 0.11433071498151929859e-3_f64 * t99000;
    let t99002 = t93034 * t4430;
    let t99004 = 0.4065600224742826258e-4_f64 * t92991;
    let t99006 = t2661 * t25227 * t14861;
    let t99007 = 0.28582678745379824648e-4_f64 * t99006;
    let t99008 = t98984 + 0.34299214494455789578e-2_f64 * t98985 - 0.25724410870841842183e-2_f64 * t98989 + t98992 + 0.17149607247227894789e-2_f64 * t98993 - t98995 / 48.0_f64 + 0.85748036236139473945e-2_f64 * t98997 - t99001 + 0.27104001498285508387e-3_f64 * t99002 - t92989 + t99004 + t99007;
    t99008
}
