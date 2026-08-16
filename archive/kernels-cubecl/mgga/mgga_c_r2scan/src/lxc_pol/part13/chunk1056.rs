//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1056/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1056<F: Float>(t1266: F, t550: F, t1104: F, t3429: F, t58: F, t875: F, t3446: F, t766: F, t10977: F, t10981: F, t37364: F, t10950: F, t11015: F, t3434: F) -> (F, F, F, F, F, F) {
    let t37470 = t550 * t1266;
    let t37472 = t3429 * t37470 * t1104;
    let t37473 = F::cast_from(0.44715219694310041527e-2_f64) * t37472;
    let t37475 = t1266 * t875 * t58;
    let t37477 = t3446 * t37475 * t766;
    let t37480 = t37364 * t10977 * t10981;
    let t37481 = F::cast_from(0.13010691197123848594e-3_f64) * t37480;
    let t37483 = t3434 * t11015 * t10950;
    (t37470, t37473, t37475, t37477, t37481, t37483)
}
