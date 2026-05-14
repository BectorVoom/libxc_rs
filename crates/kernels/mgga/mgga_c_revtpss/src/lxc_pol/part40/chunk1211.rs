//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1211/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1211<F: Float>(t1041: F, t16163: F, t2862: F, t4823: F, t1042: F, t1651: F, t3181: F, t2853: F, t15100: F, t15103: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t15519: F, t15522: F, t15524: F, t15528: F, t15530: F, t15536: F, t15540: F, t15545: F) -> (F, F, F, F) {
    let t16165 = 0.28582678745379824648e-3 * t1041 * t16163;
    let t16166 = t4823 * t2862;
    let t16167 = t1042 * t16166;
    let t16170 = t3181 * t1651;
    let t16171 = t16170 * t2853;
    let t16172 = t1042 * t16171;
    let t16179 = t15519 + t15522 + t15100 - t15103 - t15524 - t15528 + t15530 - t15536 + t15540 - t15545 - t15377 + t15379 - t15382 - t15385 - t15388 + t15392 + t15395;
    (t16165, t16167, t16172, t16179)
}
