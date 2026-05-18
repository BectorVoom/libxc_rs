//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1211/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1211<F: Float>(t10615: F, t11518: F, t3262: F, t10918: F, t11475: F, t11515: F, t11523: F, t11550: F, t11514: F, t1551: F, t3579: F, t113: F, t36985: F, t97: F) -> (F, F, F, F, F, F) {
    let t40536 = F::new(15.0) / F::new(8.0) * t3262 * t10615 * t11518;
    let t40539 = F::new(3.0) / F::new(2.0) * t3262 * t10918 * t11475;
    let t40541 = t11523 * t11515 / F::new(2.0);
    let t40544 = F::new(3.0) / F::new(2.0) * t3262 * t10918 * t11550;
    let t40547 = t3579 * t1551 * t11514 / F::new(4.0);
    let t40549 = t97 * t36985 * t113;
    (t40536, t40539, t40541, t40544, t40547, t40549)
}
