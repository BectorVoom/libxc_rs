//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 262/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk262<F: Float>(t1253: F, t1254: F, t1222: F, t1227: F, t365: F, t45: F, t370: F, t1246: F, t1238: F, t1243: F, t1251: F) -> (F, F, F, F, F, F) {
    let t1255 = t1253 * t1254;
    let t1258 = 0.92708333333333333333e-2 * t1222;
    let t1260 = -t1258 - 0.92708333333333333333e-2 * t1227;
    let t1264 = t45 * t365;
    let t1265 = t370 * t370;
    let t1266 = 1.0 / t1265;
    let t1268 = 0.301925e0 * t1222;
    let t1271 = 0.16557e0 * t1246;
    let t1273 = 0.258925e1 * t1238 - t1268 - 0.301925e0 * t1227 + 0.16504875e0 * t1243 - t1271 - 0.16557e0 * t1251;
    (t1255, t1260, t1264, t1265, t1266, t1273)
}
