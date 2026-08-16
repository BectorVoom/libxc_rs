//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1085/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1085(t9131: f64, t9139: f64, t9143: f64, t9154: f64, t9160: f64, t9166: f64, t9172: f64, t9174: f64, t9176: f64, t9178: f64, t7937: f64, t7946: f64, t8304: f64, t9653: f64) -> (f64, f64, f64, f64) {
    let t42328 = 0.79828278012425390428e-1_f64 * t9131;
    let t42332 = 0.17025839957319135759e-4_f64 * t9139;
    let t42333 = 0.85129199786595678796e-5_f64 * t9143;
    let t42335 = 0.25538759935978703638e-4_f64 * t9154;
    let t42336 = 0.25538759935978703638e-4_f64 * t9160;
    let t42337 = 0.85129199786595678796e-5_f64 * t9166;
    let t42338 = 0.85129199786595678796e-5_f64 * t9172;
    let t42339 = 0.11974241701863808564e0_f64 * t9174;
    let t42340 = 0.11974241701863808564e0_f64 * t9176;
    let t42341 = 0.79828278012425390428e-1_f64 * t9178;
    let t42343 = t9653 - t42335 + t42336 + t42337 + t42338 - t42339 - t42340 + t42341 + 0.14408463291498358381e-2_f64 * t7937 - t8304 + t7946;
    (t42328, t42332, t42333, t42343)
}
