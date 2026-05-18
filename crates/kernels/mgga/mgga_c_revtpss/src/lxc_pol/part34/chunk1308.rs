//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1308/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1308<F: Float>(t29502: F, t4248: F, t2014: F, t22483: F, t7934: F, t1497: F, t29547: F, t77: F, t1493: F, t5816: F, t22656: F, t84: F) -> (F, F, F, F, F) {
    let t114230 = F::new(12.0) * t4248 * t29502;
    let t114238 = F::new(3.0) * t2014 * t7934 * t22483;
    let t114246 = t77 * t29547 * t1497;
    let t114260 = t77 * t1493 * t5816;
    let t114264 = t77 * t84 * t22656;
    (t114230, t114238, t114246, t114260, t114264)
}
