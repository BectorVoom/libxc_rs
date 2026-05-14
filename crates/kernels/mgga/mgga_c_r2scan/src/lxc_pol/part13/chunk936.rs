//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 936/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk936<F: Float>(t1559: F, t283: F, t2298: F, t358: F, t364: F, t2316: F, t818: F, t8353: F, t1275: F, t2376: F, t1004: F, t6660: F, t2333: F, t8299: F, t19026: F, t986: F) -> (F, F, F, F, F, F, F, F) {
    let t23038 = t1559 * t1559;
    let t23040 = 1.0 / t283 / t23038;
    let t23099 = t2298 * t2298;
    let t23102 = t358 / t364 / t23099;
    let t23193 = t2316 * t2316;
    let t23194 = 1.0 / t23193;
    let t23353 = t8353 * t818;
    let t23495 = t2376 * t1275;
    let t23498 = t1004 * t6660;
    let t23754 = t8299 * t2333;
    let t23791 = t986 * t19026;
    (t23040, t23102, t23194, t23353, t23495, t23498, t23754, t23791)
}
