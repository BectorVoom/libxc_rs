//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1282/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1282(t3147: f64, t6226: f64, t6502: f64, t6320: f64, t8219: f64, t2313: f64, t8020: f64, t898: f64, t1184: f64, t2240: f64, t6327: f64, t237: f64, t6323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22492 = 0.5848223622634646207e0_f64 * t3147 * t6226;
    let t22494 = 0.35089341735807877242e1_f64 * t3147 * t6502;
    let t22496 = 6.0_f64 * t8219 * t6320;
    let t22499 = 0.35089341735807877242e1_f64 * t898 * t8020 * t2313;
    let t22500 = t2240 * t1184;
    let t22502 = 18.0_f64 * t22500 * t6327;
    let t22503 = t237 * t6323;
    (t22492, t22494, t22496, t22499, t22502, t22503)
}
