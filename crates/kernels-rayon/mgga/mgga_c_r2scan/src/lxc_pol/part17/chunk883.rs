//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 883/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk883(t2115: f64, t9422: f64, t1604: f64, t529: f64, t538: f64, t8692: f64, t2201: f64, t2687: f64, t2837: f64, t2184: f64, t535: f64, t6400: f64, t6408: f64, t6415: f64, t6420: f64, t6424: f64, t8158: f64, t8163: f64, t8167: f64, t8178: f64, t8189: f64, t9409: f64, t9416: f64, t9420: f64) -> (f64, f64) {
    let t9423 = t2115 * t9422;
    let t9424 = t1604 * t9423;
    let t9427 = t529 * t538 * t8692;
    let t9431 = t2201 * t2837 * t2687;
    let t9433 = 0.34930954652346593433e-1_f64 * t8158 + t8163 + t8167 + 0.17336443480108537126e0_f64 * t2184 * t9409 - t8178 + 0.679213007128961539e-1_f64 * t6400 + 0.29272321618148349056e-1_f64 * t6408 + t6415 - 0.32927245914677557994e-1_f64 * t6420 + t6424 - 0.34930954652346593435e-1_f64 * t9416 - 0.17465477326173296717e-1_f64 * t9420 + 0.27439371595564631661e-2_f64 * t9424 - 0.27439371595564631661e-1_f64 * t535 * t9427 + t8189 - 0.11643651550782197811e-1_f64 * t9431;
    (t9423, t9433)
}
