//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1084/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1084<F: Float>(t1181: F, t34278: F, t5641: F, t599: F, t5944: F, t604: F, t8463: F, t5645: F, t2001: F, t5534: F, t5559: F, t1165: F, t7351: F) -> (F, F, F, F, F, F) {
    let t38732 = t34278 * t1181 * t599 * t5641;
    let t38736 = t8463 * t1181 * t604 * t5944;
    let t38740 = t8463 * t1181 * t599 * t5645;
    let t38743 = t2001 * t5534;
    let t38747 = t8463 * t1181 * t604 * t5559;
    let t38751 = t8463 * t1165 * t7351 * t5944;
    (t38732, t38736, t38740, t38743, t38747, t38751)
}
