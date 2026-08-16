//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1211/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1211(t107133: f64, t107135: f64, t107139: f64, t107143: f64, t107145: f64, t107147: f64, t107151: f64, t107159: f64, t107164: f64, t107169: f64, t107174: f64, t107178: f64, t84520: f64, t84533: f64, t91305: f64, t91312: f64, t91323: f64, t91346: f64, t97378: f64, t97380: f64) -> f64 {
    let t107842 = -t107133 / 192.0_f64 - t107135 / 64.0_f64 - t84520 - 0.13565246047631171326e0_f64 * t107139 - 0.14534192193890540707e-1_f64 * t107143 + t107145 / 32.0_f64 - 5.0_f64 / 64.0_f64 * t107147 + 119.0_f64 / 1152.0_f64 * t91305 - 0.31625325607076639502e-2_f64 * t91312 + t107151 / 128.0_f64 + 7.0_f64 / 384.0_f64 * t97378 - 7.0_f64 / 192.0_f64 * t97380 + 0.60559134141210586279e-3_f64 * t91323 + 0.72670960969452703536e-2_f64 * t107159 + 0.72670960969452703536e-2_f64 * t107164 - 0.50869672678616892475e-1_f64 * t107169 + 0.10093189023535097713e-3_f64 * t91346 - t84533 + 0.72670960969452703536e-2_f64 * t107174 - 0.12111826828242117256e-2_f64 * t107178;
    t107842
}
