//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1393/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1393<F: Float>(t1333: F, t35183: F, t23034: F, t415: F, t9687: F, t23039: F, t717: F, t35180: F, t34173: F, t6961: F, t2509: F, t6966: F, t2537: F, t6945: F, t35155: F, t9660: F) -> (F, F, F, F, F, F, F, F) {
    let t122054 = t1333 * t35183;
    let t122060 = t415 * t9687 * t23034;
    let t122063 = t415 * t717 * t23039;
    let t122065 = t1333 * t35180;
    let t122068 = t415 * t34173 * t6961;
    let t122071 = t415 * t2509 * t6966;
    let t122074 = t415 * t6945 * t2537;
    let t122076 = t35155 * t9660;
    (t122054, t122060, t122063, t122065, t122068, t122071, t122074, t122076)
}
