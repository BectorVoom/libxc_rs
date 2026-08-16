//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1493/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1493<F: Float>(t14220: F, t48007: F, t22331: F, t2470: F, t4101: F, t10073: F, t22369: F, t136: F, t2457: F, t47429: F, t6862: F, t22351: F, t2439: F, t2777: F) -> (F, F, F, F, F) {
    let t75005 = t48007 * t14220;
    let t75021 = t4101 * t22331 * t2470;
    let t75026 = t10073 * t22369;
    let t75068 = t47429 * t6862 * t136 * t2457;
    let t75074 = t2439 * t2777 * t22351;
    (t75005, t75021, t75026, t75068, t75074)
}
