//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 779/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk779<F: Float>(t1572: F, t6097: F, t2079: F, t4358: F, t1571: F, t1347: F, t1911: F, t1354: F, t2084: F, t1356: F, t5613: F, t1919: F, t3947: F) -> (F, F, F, F, F, F, F) {
    let t6098 = t6097 * t1572;
    let t6101 = t2079 * t4358;
    let t6102 = t6101 * t1571;
    let t6106 = t1911 * t1347;
    let t6111 = t2084 * t1354;
    let t6114 = t5613 * t1356;
    let t6117 = t1919 * t3947;
    (t6098, t6101, t6102, t6106, t6111, t6114, t6117)
}
