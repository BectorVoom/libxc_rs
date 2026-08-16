//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1773/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1773<F: Float>(t25296: F, t7058: F, t2453: F, t7057: F, t136: F, t1958: F, t2457: F) -> (F, F, F, F) {
    let t25297 = t7058 * t25296;
    let t25299 = t2453 * t7057;
    let t25300 = t1958 * t136;
    let t25301 = t25300 * t2457;
    (t25297, t25299, t25300, t25301)
}
