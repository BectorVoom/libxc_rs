//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1265/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1265<F: Float>(t3431: F, t5717: F, t1163: F, t1165: F, t4298: F, t6403: F, t1181: F, t22040: F, t3361: F, t4643: F, t5122: F, t5852: F) -> (F, F, F, F) {
    let t23351 = t3431 * t5717;
    let t23355 = t1163 * t1165 * t4298 * t6403;
    let t23359 = t3361 * t1181 * t4643 * t22040;
    let t23363 = t1163 * t1181 * t5852 * t5122;
    (t23351, t23355, t23359, t23363)
}
