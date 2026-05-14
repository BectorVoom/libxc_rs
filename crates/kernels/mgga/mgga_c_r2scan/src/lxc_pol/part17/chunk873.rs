//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 873/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk873<F: Float>(t3574: F, t792: F, t2333: F, t910: F, t795: F, t105: F, t920: F, t97: F) -> (F, F, F, F, F) {
    let t11486 = t3574 * t792;
    let t11496 = t2333 * t910;
    let t11497 = t11496 * t795;
    let t11505 = t105 * t920;
    let t11506 = t97 * t11505;
    (t11486, t11496, t11497, t11505, t11506)
}
