//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 690/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk690<F: Float>(t1626: F, t501: F, t572: F, t81: F, t79: F, t127: F, t4803: F, t500: F, t78: F, t1503: F, t4913: F, t541: F, t555: F, t1511: F, t1639: F, t4911: F, t4915: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5130 = 12.0 * t501 * t1626;
    let t5135 = t81 * t572;
    let t5136 = 1.0 / t5135;
    let t5137 = t79 * t5136;
    let t5139 = 120.0 * t5137 * t127;
    let t5141 = 24.0 * t4803 * t127;
    let t5142 = t78 * t500;
    let t5143 = t5142 * t127;
    let t5144 = 144.0 * t5143;
    let t5146 = t1503 * t4913 * t541;
    let t5148 = 0.35089341735807877242e1 * t555 * t5146;
    let t5149 = t1511 * t1639;
    let t5152 = t4911 * t4913 * t4915;
    (t5130, t5135, t5136, t5137, t5139, t5141, t5142, t5143, t5144, t5146, t5148, t5149, t5152)
}
