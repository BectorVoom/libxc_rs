//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1043/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1043(t17161: f64, t2826: f64, t136: f64, t10304: f64, t17152: f64, t17167: f64, t908: f64, t17171: f64, t17183: f64, t17178: f64, t10556: f64, t10577: f64, t13598: f64, t13600: f64, t13601: f64, t13603: f64, t17149: f64, t17154: f64, t17159: f64, t17163: f64, t17165: f64, t17169: f64, t17173: f64, t17175: f64, t17180: f64, t17185: f64, t17189: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
    let t17271 = -t10577 - 4.0_f64 / 27.0_f64 * t10556 - 8.0_f64 / 27.0_f64 * t13598 + t13600 - t13601 + t13603 + 2.0_f64 / 27.0_f64 * t17149 - 10.0_f64 / 27.0_f64 * t17154 + 4.0_f64 / 3.0_f64 * t17159 - 4.0_f64 / 9.0_f64 * t17163 - 2.0_f64 / 9.0_f64 * t17165 - 2.0_f64 * t17169 + 4.0_f64 / 3.0_f64 * t17173 + t17175 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t17180 + 2.0_f64 / 3.0_f64 * t17185 - t17189 / 3.0_f64;
    (t17241, t17244, t17247, t17250, t17253, t17256, t17271)
}
