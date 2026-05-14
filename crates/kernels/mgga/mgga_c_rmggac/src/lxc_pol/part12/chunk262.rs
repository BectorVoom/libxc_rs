//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 262/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk262<F: Float>(t338: F, t866: F, t118: F, t1253: F, t1255: F, t1257: F, t1260: F, t1263: F, t1265: F, t1268: F, t1271: F, t82: F, t73: F, t75: F, t80: F, t295: F, t299: F) -> (F, F, F, F, F, F) {
    let t1273 = t338 * t866;
    let t1274 = t118 * t1273;
    let t1276 = -0.11974241701863808564e0 * t1253 + 0.35922725105591425692e0 * t1255 + 0.11974241701863808564e0 * t1257 - 0.59871208509319042821e-1 * t1260 - 0.23948483403727617128e0 * t1263 - 0.11974241701863808564e0 * t1265 + 0.59871208509319042821e-1 * t1268 - 0.39914139006212695214e-1 * t1271 + 0.19957069503106347607e-1 * t1274;
    let t1277 = t82 * t1276;
    let t1279 = t75 * t73;
    let t1281 = 132.0 * t1279 * t80;
    let t1283 = 288.0 * t295 * t299;
    (t1274, t1276, t1277, t1279, t1281, t1283)
}
