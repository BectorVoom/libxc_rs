//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 275/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk275<F: Float>(t1222: F, t1246: F, t1227: F, t1238: F, t1243: F, t1251: F, t373: F) -> (F, F, F, F) {
    let t1268 = 0.301925e0 * t1222;
    let t1271 = 0.16557e0 * t1246;
    let t1273 = 0.258925e1 * t1238 - t1268 - 0.301925e0 * t1227 + 0.16504875e0 * t1243 - t1271 - 0.16557e0 * t1251;
    let t1275 = 1.0 / t373;
    (t1268, t1271, t1273, t1275)
}
