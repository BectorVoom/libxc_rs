//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1477/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1477<F: Float>(t10226: F, t106: F, t107: F, t10: F, t580: F, t22: F, t576: F, t15: F, t588: F, t11: F, t2: F, t2224: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10227 = F::new(1.0) / t10226;
    let t10240 = t107 * t106;
    let t10241 = F::new(1.0) / t10240;
    let t10270 = t10 * t580;
    let t10271 = F::new(12.0) * t10270;
    let t10272 = t576 * t22;
    let t10273 = F::new(36.0) * t10272;
    let t10275 = F::new(24.0) * t15 * t588;
    let t10276 = t11 * t2;
    let t10278 = F::new(24.0) * t10276 * t22;
    let t10279 = t2224 * t588;
    (t10227, t10241, t10270, t10271, t10272, t10273, t10275, t10276, t10278, t10279)
}
