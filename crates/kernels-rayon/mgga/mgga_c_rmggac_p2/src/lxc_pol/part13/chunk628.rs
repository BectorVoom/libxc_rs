//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 628/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk628(t8252: f64, t8290: f64, t82: f64, t72: f64, t1356: f64, t8265: f64, t8281: f64, t884: f64, t739: f64, t8273: f64, t7924: f64, t7945: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8291 = t8252 + t8290;
    let t8292 = t82 * t8291;
    let t8293 = t72 * t8292;
    let t8294 = t1356 * t8265;
    let t8295 = 0.79828278012425390428e-1_f64 * t8294;
    let t8296 = t884 * t8281;
    let t8297 = 0.11974241701863808564e0_f64 * t8296;
    let t8298 = t739 * t8273;
    let t8299 = 0.11974241701863808564e0_f64 * t8298;
    let t8301 = 0.5987120850931904282e-1_f64 * t7924;
    let t8305 = 0.85129199786595678799e-5_f64 * t7945;
    (t8291, t8292, t8293, t8295, t8297, t8299, t8301, t8305)
}
