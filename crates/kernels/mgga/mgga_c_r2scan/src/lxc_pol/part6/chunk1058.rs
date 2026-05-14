//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1058/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1058<F: Float>(t322: F, t8397: F, t2394: F, t833: F, t1013: F, t1299: F, t1295: F, t829: F, t1292: F, t1300: F, t2397: F, t327: F, t6693: F, t834: F, t1018: F, t1305: F) -> (F, F, F, F, F, F) {
    let t324 = 0.0 < t322;
    let t332 = 0.25e1 < t322;
    let t8398 = piecewise3(t324, 0.0, t8397);
    let t8401 = t2394 * t833;
    let t8404 = t1013 * t1299;
    let t8409 = t1013 * t1295;
    let t8412 = t2394 * t829;
    let t8415 = t1013 * t1292;
    let t8420 = -0.64e0 * t8398 * t327 - 0.256e1 * t8401 * t829 - 0.384e1 * t8404 * t1295 - 0.128e1 * t2397 * t1292 - 0.384e1 * t6693 * t8409 - 0.256e1 * t1300 * t8412 - 0.128e1 * t1300 * t8415 - 0.64e0 * t834 * t8398;
    let t8425 = t1018 * t1305;
    let t8438 = piecewise3(t332, 0.0, t8397);
    (t8398, t8401, t8404, t8420, t8425, t8438)
}
