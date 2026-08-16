//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1144/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1144(t18672: f64, t4939: f64, t330: f64, t6533: f64, t829: f64, t9924: f64, t25: f64, t6540: f64, t285: f64, t14538: f64, t19219: f64, t19223: f64, t19226: f64, t19229: f64, t19233: f64, t19236: f64, t2872: f64, t6522: f64, t6541: f64, t984: f64, t991: f64) -> f64 {
    let t19239 = t4939 * t18672;
    let t19242 = t6533 * t330;
    let t19243 = t19242 * t829;
    let t19244 = t9924 * t19243;
    let t19249 = t25 * t6540;
    let t19250 = t285 * t19249;
    let t19252 = t2872 * t6522 / 54.0_f64 - t991 * t19219 / 72.0_f64 + t991 * t19223 / 144.0_f64 + t991 * t19226 / 48.0_f64 - t991 * t19229 / 36.0_f64 - t991 * t19233 / 288.0_f64 - t991 * t19236 / 144.0_f64 + t991 * t19239 / 216.0_f64 + t991 * t19244 / 144.0_f64 - t14538 + t984 * t6541 / 36.0_f64 - t19250 / 288.0_f64;
    t19252
}
