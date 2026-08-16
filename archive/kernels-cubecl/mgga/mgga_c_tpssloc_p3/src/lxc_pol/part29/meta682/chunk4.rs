//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2308/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2308<F: Float>(t24658: F, t27683: F, t1184: F, t24682: F, t27607: F, t1209: F, t85821: F, t1215: F, t15555: F, t15612: F, t15650: F, t15704: F, t24655: F, t24664: F, t24670: F, t24716: F, t24729: F, t24736: F, t27684: F, t478: F, t4974: F, t4980: F, t5014: F, t7345: F, t7376: F, t86140: F, t86327: F) -> F {
    let t95295 = t24658 * t27683;
    let t95303 = t24682 * t27607 * t1184;
    let t95304 = t85821 * t1209;
    let t95316 = t24716 * t5014 / F::cast_from(768.0_f64) + t86140 * t4980 / F::cast_from(384.0_f64) + t24729 * t15555 / F::cast_from(384.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t27684 * t24655 - F::cast_from(0.20186378047070195428e-3_f64) * t95295 * t24664 + F::cast_from(0.10093189023535097714e-3_f64) * t95295 * t24670 + t86327 * t15704 / F::cast_from(1152.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t95303 * t95304 * t478 * t1215 * t7376 - t24736 * t4974 / F::cast_from(576.0_f64) - t7345 * t15650 / F::cast_from(576.0_f64) - t7345 * t15612 / F::cast_from(1152.0_f64);
    t95316
}
