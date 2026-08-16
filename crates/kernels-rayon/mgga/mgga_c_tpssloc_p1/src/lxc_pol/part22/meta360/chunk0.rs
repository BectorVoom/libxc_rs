//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1597/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1597(t17161: f64, t2826: f64, t136: f64, t10304: f64, t17152: f64, t17167: f64, t908: f64, t17171: f64, t17183: f64, t17178: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17240 = t2826 * t17161;
    let t17241 = t136 * t17240;
    let t17243 = t10304 * t17152;
    let t17244 = t136 * t17243;
    let t17246 = t908 * t17167;
    let t17247 = t136 * t17246;
    let t17249 = t908 * t17171;
    let t17250 = t136 * t17249;
    let t17252 = t908 * t17183;
    let t17253 = t136 * t17252;
    let t17255 = t2826 * t17178;
    let t17256 = t136 * t17255;
    (t17240, t17241, t17243, t17244, t17246, t17247, t17249, t17250, t17252, t17253, t17255, t17256)
}
