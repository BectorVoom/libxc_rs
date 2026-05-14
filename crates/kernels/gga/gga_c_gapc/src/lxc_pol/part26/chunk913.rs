//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 913/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk913<F: Float>(t103: F, t2232: F, t7062: F, t880: F, t4914: F, t572: F, t10408: F, t883: F, t1066: F, t268: F, t8449: F, t332: F, t875: F, t9739: F, t147: F, t19: F, t2315: F, t3295: F, t966: F) -> (F, F, F, F, F, F, F, F) {
    let t23678 = t2232 * t103;
    let t23723 = t880 * t7062;
    let t23726 = t572 * t4914;
    let t24004 = t10408 * t883;
    let t24007 = t1066 * t7062;
    let t24081 = t8449 * t268;
    let t24086 = t9739 * t332 * t103 * t875;
    let t24092 = t3295 * t966 * t2315 * t19 * t147;
    (t23678, t23723, t23726, t24004, t24007, t24081, t24086, t24092)
}
