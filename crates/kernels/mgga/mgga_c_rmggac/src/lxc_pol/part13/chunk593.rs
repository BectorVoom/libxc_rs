//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 593/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk593<F: Float>(t8252: F, t8290: F, t82: F, t72: F, t1356: F, t8265: F, t8281: F, t884: F, t739: F, t8273: F, t7924: F, t7945: F, t8258: F, t8278: F, t2265: F, t942: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8291 = t8252 + t8290;
    let t8292 = t82 * t8291;
    let t8293 = t72 * t8292;
    let t8294 = t1356 * t8265;
    let t8295 = 0.79828278012425390428e-1 * t8294;
    let t8296 = t884 * t8281;
    let t8297 = 0.11974241701863808564e0 * t8296;
    let t8298 = t739 * t8273;
    let t8299 = 0.11974241701863808564e0 * t8298;
    let t8301 = 0.5987120850931904282e-1 * t7924;
    let t8305 = 0.85129199786595678799e-5 * t7945;
    let t8306 = t1356 * t8258;
    let t8307 = 0.39914139006212695214e-1 * t8306;
    let t8308 = t884 * t8278;
    let t8309 = 0.59871208509319042821e-1 * t8308;
    let t8310 = t942 * t2265;
    (t8291, t8292, t8293, t8295, t8297, t8299, t8301, t8305, t8307, t8309, t8310)
}
