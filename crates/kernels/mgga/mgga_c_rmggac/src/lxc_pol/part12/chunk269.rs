//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 269/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk269<F: Float>(t128: F, t848: F, t326: F, t874: F, t876: F, t118: F, t338: F, t866: F, t1253: F, t1255: F, t1257: F, t1260: F, t1263: F, t1265: F) -> (F, F, F, F) {
    let t1267 = t128 * t848;
    let t1268 = t326 * t1267;
    let t1270 = t874 * t876;
    let t1271 = t118 * t1270;
    let t1273 = t338 * t866;
    let t1274 = t118 * t1273;
    let t1276 = -F::cast_from(0.11974241701863808564e0_f64) * t1253 + F::cast_from(0.35922725105591425692e0_f64) * t1255 + F::cast_from(0.11974241701863808564e0_f64) * t1257 - F::cast_from(0.59871208509319042821e-1_f64) * t1260 - F::cast_from(0.23948483403727617128e0_f64) * t1263 - F::cast_from(0.11974241701863808564e0_f64) * t1265 + F::cast_from(0.59871208509319042821e-1_f64) * t1268 - F::cast_from(0.39914139006212695214e-1_f64) * t1271 + F::cast_from(0.19957069503106347607e-1_f64) * t1274;
    (t1268, t1271, t1274, t1276)
}
