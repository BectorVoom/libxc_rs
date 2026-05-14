//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 737/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk737<F: Float>(t10259: F, t508: F, t3813: F, t670: F, t10: F, t580: F, t22: F, t576: F, t15: F, t588: F, t11: F, t2: F, t2224: F, t27: F, t584: F, t20: F, t596: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10260 = t508 * t10259;
    let t10263 = t3813 * t670;
    let t10270 = t10 * t580;
    let t10271 = 12.0 * t10270;
    let t10272 = t576 * t22;
    let t10273 = 36.0 * t10272;
    let t10275 = 24.0 * t15 * t588;
    let t10276 = t11 * t2;
    let t10278 = 24.0 * t10276 * t22;
    let t10279 = t2224 * t588;
    let t10280 = 144.0 * t10279;
    let t10281 = t584 * t27;
    let t10282 = 240.0 * t10281;
    let t10284 = 120.0 * t20 * t596;
    (t10260, t10263, t10271, t10273, t10275, t10278, t10280, t10282, t10284)
}
