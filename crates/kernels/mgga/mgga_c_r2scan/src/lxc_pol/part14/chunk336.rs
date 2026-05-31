//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 336/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk336<F: Float>(t1234: F, t471: F, t97: F, t292: F, t800: F, t297: F, t806: F, t1218: F, t298: F, t307: F, t810: F, t308: F, rho0: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1236 = t97 * t471 * t1234;
    let t1237 = F::cast_from(3.0_f64) * t1236;
    let t1242 = F::cast_from(1.0_f64) / t292 / t800 / rho0;
    let t1243 = tau0 * t1242;
    let t1248 = F::cast_from(1.0_f64) / t297;
    let t1249 = t806 * t806;
    let t1250 = t1248 * t1249;
    let t1253 = t298 * t1218;
    let t1256 = F::cast_from(1.0_f64) / t307;
    let t1257 = t810 * t810;
    let t1258 = t1256 * t1257;
    let t1261 = -t1218;
    let t1262 = t308 * t1261;
    (t1237, t1243, t1248, t1249, t1250, t1253, t1256, t1257, t1258, t1261, t1262)
}
