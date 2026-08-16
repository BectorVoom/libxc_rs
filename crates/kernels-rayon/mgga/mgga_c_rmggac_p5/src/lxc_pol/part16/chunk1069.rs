//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1069/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1069(t1632: f64, t9523: f64, t1635: f64, t5898: f64, t5144: f64, t5267: f64, t1356: f64, t1364: f64, t2211: f64, t26283: f64, t26287: f64, t26291: f64, t30204: f64, t34757: f64, t38749: f64, t38757: f64, t38784: f64, t4044: f64, t42740: f64, t44941: f64, t44949: f64, t44951: f64, t46846: f64, t46867: f64, t5048: f64, t6355: f64, t6394: f64, t6397: f64, t699: f64, t8041: f64, t9315: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48259 = t9523 * t1632;
    let t48262 = t9523 * t1635;
    let t48265 = t9523 * t5898;
    let t48268 = t9523 * t5144;
    let t48271 = t9523 * t5267;
    let t48274 = -0.16163010989689081288e-5_f64 * t34757 + 0.60975299583150056624e-3_f64 * t38749 - 0.60975299583150056624e-3_f64 * t38757 + t42740 + 0.11974241701863808564e0_f64 * t44941 - 0.23948483403727617128e0_f64 * t6355 * t9315 + 0.85129199786595678799e-5_f64 * t44949 - 0.212822999466489197e-4_f64 * t44951 - 0.71845450211182851384e0_f64 * t4044 * t699 * t6394 + 0.11974241701863808564e1_f64 * t5048 * t699 * t6397 + 0.47896966807455234256e0_f64 * t1364 * t2211 * t46846 - 0.23948483403727617128e0_f64 * t1356 * t8041 * t46867 + 0.40002837092893167871e0_f64 * t38784 + 0.71845450211182851384e0_f64 * t26287 * t48259 - 0.14369090042236570277e1_f64 * t26283 * t48262 - 0.71845450211182851384e0_f64 * t26291 * t48265 + 0.47896966807455234256e0_f64 * t30204 * t48268 - 0.71845450211182851384e0_f64 * t26291 * t48271;
    (t48259, t48262, t48265, t48268, t48271, t48274)
}
