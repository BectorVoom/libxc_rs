//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1031/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1031<F: Float>(t1927: F, t644: F, t4144: F, t9593: F, t196: F, t197: F, t3821: F, t2394: F, t30: F, t2411: F, t14365: F, t605: F, t775: F) -> (F, F, F, F, F, F, F) {
    let t25163 = t1927 * t644;
    let t25177 = t9593 * t4144;
    let t25188 = t3821 * t196 * t197;
    let t25198 = t30 * t2394;
    let t25207 = t2411 * t30;
    let t25208 = t25207 * t14365;
    let t25211 = t605 * t775;
    (t25163, t25177, t25188, t25198, t25207, t25208, t25211)
}
