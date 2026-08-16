//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2064/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2064(t99085: f64, t2689: f64, t27239: f64, t25277: f64, t4458: f64, t14685: f64, t14756: f64, t7021: f64, t14760: f64, t93015: f64, t27316: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99086 = 0.10164000561857065645e-3_f64 * t99085;
    let t99091 = t2689 * t27239;
    let t99099 = t25277 * t4458;
    let t99100 = 7.0_f64 / 72.0_f64 * t99099;
    let t99102 = t7021 * t14685 * t14756;
    let t99103 = 7.0_f64 / 24.0_f64 * t99102;
    let t99113 = t93015 * t14760;
    let t99125 = t27316 * t72 * t686;
    (t99086, t99091, t99100, t99103, t99113, t99125)
}
