//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 364/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk364<F: Float>(t1452: F, t1435: F, t1398: F, t32: F, t5: F) -> (F, F, F) {
    let t1453 = 1.0 / t1452;
    let t1454 = t1435 * t1453;
    let t1458 = t5 * t1398 * t32;
    let t1459 = 0.14764627977777777777e-2 * t1458;
    (t1453, t1454, t1459)
}
