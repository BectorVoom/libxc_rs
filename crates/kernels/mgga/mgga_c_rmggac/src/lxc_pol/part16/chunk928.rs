//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 928/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk928<F: Float>(t1632: F, t9523: F, t1635: F, t5898: F, t5144: F, t5267: F, t1356: F, t1364: F, t2211: F, t26283: F, t26287: F, t26291: F, t30204: F, t34757: F, t38749: F, t38757: F, t38784: F, t4044: F, t42740: F, t44941: F, t44949: F, t44951: F, t46846: F, t46867: F, t5048: F, t6355: F, t6394: F, t6397: F, t699: F, t8041: F, t9315: F) -> (F, F, F, F, F, F) {
    let t48259 = t9523 * t1632;
    let t48262 = t9523 * t1635;
    let t48265 = t9523 * t5898;
    let t48268 = t9523 * t5144;
    let t48271 = t9523 * t5267;
    let t48274 = -0.16163010989689081288e-5 * t34757 + 0.60975299583150056624e-3 * t38749 - 0.60975299583150056624e-3 * t38757 + t42740 + 0.11974241701863808564e0 * t44941 - 0.23948483403727617128e0 * t6355 * t9315 + 0.85129199786595678799e-5 * t44949 - 0.212822999466489197e-4 * t44951 - 0.71845450211182851384e0 * t4044 * t699 * t6394 + 0.11974241701863808564e1 * t5048 * t699 * t6397 + 0.47896966807455234256e0 * t1364 * t2211 * t46846 - 0.23948483403727617128e0 * t1356 * t8041 * t46867 + 0.40002837092893167871e0 * t38784 + 0.71845450211182851384e0 * t26287 * t48259 - 0.14369090042236570277e1 * t26283 * t48262 - 0.71845450211182851384e0 * t26291 * t48265 + 0.47896966807455234256e0 * t30204 * t48268 - 0.71845450211182851384e0 * t26291 * t48271;
    (t48259, t48262, t48265, t48268, t48271, t48274)
}
