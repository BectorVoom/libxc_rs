//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 948/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk948(t9235: f64, t9265: f64, t165: f64, t2531: f64, t782: f64, t826: f64, t2533: f64, t2626: f64, t781: f64, t142: f64, t2539: f64, t2538: f64) -> (f64, f64, f64, f64, f64) {
    let t9266 = t9235 + t9265;
    let t9267 = t9266 * t165;
    let t9268 = t2531 * t782;
    let t9269 = t9268 * t826;
    let t9270 = 3.0_f64 * t9269;
    let t9271 = t2533 * t2626;
    let t9272 = 3.0_f64 * t9271;
    let t9273 = t781 * t781;
    let t9274 = 1.0_f64 / t9273;
    let t9275 = t142 * t9274;
    let t9276 = t2539 * t826;
    let t9277 = t9275 * t9276;
    let t9278 = 6.0_f64 * t9277;
    let t9279 = t826 * t2626;
    let t9280 = t2538 * t9279;
    (t9267, t9270, t9272, t9278, t9280)
}
