//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1178/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1178<F: Float>(t1998: F, t6125: F, t30811: F, t6090: F, t30543: F, t9670: F, t1165: F, t39794: F, t604: F, t7413: F, t1181: F, t30856: F, t40215: F, t599: F) -> (F, F, F, F, F) {
    let t40387 = t1998 * t6125;
    let t40390 = t30811 * t6090;
    let t40398 = t30543 * t9670;
    let t40403 = t7413 * t1165 * t604 * t39794;
    let t40408 = t30856 * t1181 * t599 * t40215;
    (t40387, t40390, t40398, t40403, t40408)
}
