//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 855/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk855(t42246: f64, t36912: f64, t9082: f64, t36935: f64, t2185: f64, t678: f64, t9086: f64, t8825: f64, t8852: f64, t8856: f64, t8860: f64, t8864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42247 = 0.72042316457491791906e-3_f64 * t42246;
    let t42248 = t36912 * t9082;
    let t42250 = t36935 * t9082;
    let t42258 = t9086 * t2185 * t678;
    let t42259 = 0.19863479950205658386e-4_f64 * t42258;
    let t42282 = 0.11974241701863808564e0_f64 * t8825;
    let t42289 = 0.30487649791575028314e-3_f64 * t8852;
    let t42290 = 0.30487649791575028314e-3_f64 * t8856;
    let t42291 = 0.30487649791575028314e-3_f64 * t8860;
    let t42292 = 0.30487649791575028314e-3_f64 * t8864;
    (t42247, t42248, t42250, t42259, t42282, t42289, t42290, t42291, t42292)
}
