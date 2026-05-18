//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1255/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1255<F: Float>(t10073: F, t25403: F, t27198: F, t2471: F, t27202: F, t15003: F, t93194: F, t7759: F, t822: F, t2470: F, t27340: F, t25387: F) -> (F, F, F, F, F, F) {
    let t99297 = t10073 * t27198 * t25403;
    let t99307 = t27202 * t2471;
    let t99313 = t93194 * t15003;
    let t99334 = t822 * t7759;
    let t99365 = t27340 * t2470;
    let t99366 = t25387 * t99365;
    (t99297, t99307, t99313, t99334, t99365, t99366)
}
