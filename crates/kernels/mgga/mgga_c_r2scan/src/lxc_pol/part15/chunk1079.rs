//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1079/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1079<F: Float>(t10615: F, t11518: F, t3262: F, t10918: F, t11475: F, t11515: F, t11523: F, t11550: F, t11514: F, t1551: F, t3579: F, t113: F, t36985: F, t97: F, t11510: F, t23754: F, t3263: F, t3275: F) -> (F, F, F, F, F, F, F) {
    let t40536 = 15.0 / 8.0 * t3262 * t10615 * t11518;
    let t40539 = 3.0 / 2.0 * t3262 * t10918 * t11475;
    let t40541 = t11523 * t11515 / 2.0;
    let t40544 = 3.0 / 2.0 * t3262 * t10918 * t11550;
    let t40547 = t3579 * t1551 * t11514 / 4.0;
    let t40549 = t97 * t36985 * t113;
    let t40551 = 3.0 * t40549 * t11510;
    let t40554 = t3275 * t3263 * t23754 / 4.0;
    (t40536, t40539, t40541, t40544, t40547, t40551, t40554)
}
