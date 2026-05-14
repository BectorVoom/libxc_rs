//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 988/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk988<F: Float>(t2411: F, t30: F, t1946: F, t2684: F, t7043: F, t820: F, t843: F, t240: F, t7036: F, t2670: F, t7033: F, t2482: F, t27: F, t1941: F, t243: F, t2712: F, t64: F) -> (F, F, F, F, F, F, F, F) {
    let t25207 = t2411 * t30;
    let t25219 = t1946 * t2684;
    let t25220 = 0.11337795902333997111e-1 * t25219;
    let t25222 = t820 * t7043 * t843;
    let t25227 = t7036 * t240;
    let t25231 = t7033 * t2670;
    let t25232 = 0.27104001498285508387e-3 * t25231;
    let t25234 = t2482 * t7043 * t27;
    let t25237 = t1941 * t243;
    let t25240 = t64 * t2712;
    (t25207, t25220, t25222, t25227, t25232, t25234, t25237, t25240)
}
