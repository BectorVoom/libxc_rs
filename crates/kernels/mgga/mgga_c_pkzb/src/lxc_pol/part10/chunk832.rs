//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 832/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk832<F: Float>(t139: F, t24: F, t1626: F, t501: F, t1662: F, t496: F, t572: F, t81: F, t79: F, t127: F, t4803: F, t500: F, t78: F, t1503: F, t4913: F, t541: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5106 = 1.0 / t139 / t24;
    let t5130 = 12.0 * t501 * t1626;
    let t5131 = t496 * t1662;
    let t5133 = t501 * t1662;
    let t5135 = t81 * t572;
    let t5136 = 1.0 / t5135;
    let t5137 = t79 * t5136;
    let t5139 = 120.0 * t5137 * t127;
    let t5141 = 24.0 * t4803 * t127;
    let t5142 = t78 * t500;
    let t5143 = t5142 * t127;
    let t5146 = t1503 * t4913 * t541;
    (t5106, t5130, t5131, t5133, t5135, t5137, t5139, t5141, t5142, t5143, t5146)
}
