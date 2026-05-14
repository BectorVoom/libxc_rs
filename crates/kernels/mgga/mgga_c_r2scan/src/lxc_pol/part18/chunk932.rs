//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 932/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk932<F: Float>(t545: F, t7613: F, t19790: F, t495: F, t1559: F, t283: F, t2298: F, t358: F, t364: F, t2316: F, t1275: F, t2376: F, t1004: F, t6660: F, t2182: F, t775: F) -> (F, F, F, F, F, F, F, F) {
    let t22868 = t545 * t7613;
    let t22948 = t19790 * t495;
    let t23038 = t1559 * t1559;
    let t23040 = 1.0 / t283 / t23038;
    let t23099 = t2298 * t2298;
    let t23102 = t358 / t364 / t23099;
    let t23193 = t2316 * t2316;
    let t23194 = 1.0 / t23193;
    let t23495 = t2376 * t1275;
    let t23498 = t1004 * t6660;
    let t24039 = t2182 * t775;
    (t22868, t22948, t23040, t23102, t23194, t23495, t23498, t24039)
}
