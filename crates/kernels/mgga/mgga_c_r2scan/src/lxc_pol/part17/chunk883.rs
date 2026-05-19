//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 883/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk883<F: Float>(t2115: F, t9422: F, t1604: F, t529: F, t538: F, t8692: F, t2201: F, t2687: F, t2837: F, t2184: F, t535: F, t6400: F, t6408: F, t6415: F, t6420: F, t6424: F, t8158: F, t8163: F, t8167: F, t8178: F, t8189: F, t9409: F, t9416: F, t9420: F) -> (F, F) {
    let t9423 = t2115 * t9422;
    let t9424 = t1604 * t9423;
    let t9427 = t529 * t538 * t8692;
    let t9431 = t2201 * t2837 * t2687;
    let t9433 = F::cast_from(0.34930954652346593433e-1_f64) * t8158 + t8163 + t8167 + F::cast_from(0.17336443480108537126e0_f64) * t2184 * t9409 - t8178 + F::cast_from(0.679213007128961539e-1_f64) * t6400 + F::cast_from(0.29272321618148349056e-1_f64) * t6408 + t6415 - F::cast_from(0.32927245914677557994e-1_f64) * t6420 + t6424 - F::cast_from(0.34930954652346593435e-1_f64) * t9416 - F::cast_from(0.17465477326173296717e-1_f64) * t9420 + F::cast_from(0.27439371595564631661e-2_f64) * t9424 - F::cast_from(0.27439371595564631661e-1_f64) * t535 * t9427 + t8189 - F::cast_from(0.11643651550782197811e-1_f64) * t9431;
    (t9423, t9433)
}
