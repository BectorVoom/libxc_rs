//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2308/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2308(t24658: f64, t27683: f64, t1184: f64, t24682: f64, t27607: f64, t1209: f64, t85821: f64, t1215: f64, t15555: f64, t15612: f64, t15650: f64, t15704: f64, t24655: f64, t24664: f64, t24670: f64, t24716: f64, t24729: f64, t24736: f64, t27684: f64, t478: f64, t4974: f64, t4980: f64, t5014: f64, t7345: f64, t7376: f64, t86140: f64, t86327: f64) -> f64 {
    let t95295 = t24658 * t27683;
    let t95303 = t24682 * t27607 * t1184;
    let t95304 = t85821 * t1209;
    let t95316 = t24716 * t5014 / 768.0_f64 + t86140 * t4980 / 384.0_f64 + t24729 * t15555 / 384.0_f64 - 0.10093189023535097714e-3_f64 * t27684 * t24655 - 0.20186378047070195428e-3_f64 * t95295 * t24664 + 0.10093189023535097714e-3_f64 * t95295 * t24670 + t86327 * t15704 / 1152.0_f64 - 0.20186378047070195428e-3_f64 * t95303 * t95304 * t478 * t1215 * t7376 - t24736 * t4974 / 576.0_f64 - t7345 * t15650 / 576.0_f64 - t7345 * t15612 / 1152.0_f64;
    t95316
}
