//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 934/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk934<F: Float>(t322: F, t1339: F, t856: F, t1338: F, t1343: F, t352: F, t1347: F, t349: F, t6681: F, t854: F, t1305: F, t1348: F, t2438: F, t330: F, t6701: F, t6706: F, t6745: F, t837: F, t855: F) -> (F, F, F, F, F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t332 = 0.25e1 < t322;
    let t6746 = t1339 * t856;
    let t6750 = t1338 * t856;
    let t6751 = t352 * t1343;
    let t6755 = 1.0 / t1347 / t349;
    let t6759 = piecewise3(t332, t6681, 0.0);
    let t6767 = 1.0 / t1347 / t854;
    let t6772 = piecewise5(t323, 3.0 * t1305 * t837 * t330 + t6701 * t330 + t6706 * t330, t331, t6745, -0.63e1 * t1348 * t6746 * t352 - 0.63e1 * t6750 * t6751 - 0.945e1 * t6755 * t6746 * t352 - 0.105e1 * t855 * t6759 * t352 - 0.4725e1 * t1348 * t1343 * t2438 - 0.23625e1 * t6767 * t6746 * t352);
    (t6746, t6751, t6755, t6759, t6767, t6772)
}
