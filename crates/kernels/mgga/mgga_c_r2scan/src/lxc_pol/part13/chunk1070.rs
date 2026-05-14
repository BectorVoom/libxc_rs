//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1070/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1070<F: Float>(t11011: F, t11479: F, t3262: F, t11502: F, t37513: F, t10940: F, t11540: F, t38299: F, t897: F, t10680: F, t38301: F, t10918: F, t3275: F, t7040: F, t11531: F, t481: F) -> (F, F, F, F, F, F) {
    let t40404 = 3.0 / 2.0 * t3262 * t11479 * t11011;
    let t40406 = 3.0 / 4.0 * t37513 * t11502;
    let t40408 = t10940 * t11540 / 4.0;
    let t40409 = t38299 * t897;
    let t40411 = t10680 * t40409 * t38301;
    let t40415 = t3275 * t10918 * t7040 / 2.0;
    let t40416 = t11531 * t481;
    (t40404, t40406, t40408, t40411, t40415, t40416)
}
