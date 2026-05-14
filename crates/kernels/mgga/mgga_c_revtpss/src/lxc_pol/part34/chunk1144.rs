//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1144/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1144<F: Float>(t1558: F, t231: F, t6048: F, t106446: F, t106448: F, t25317: F, t25392: F, t27189: F, t27199: F, t27353: F, t29636: F, t29669: F, t29691: F, t6072: F, t7070: F, t7076: F, t76161: F, t7759: F, t93349: F, t99456: F, t99460: F, t99481: F, t99496: F, t99520: F, t99522: F) -> (F,) {
    let t113387 = t6048 * t1558 * t231;
    let t113412 = -0.28912093960683998208e-1 * t99456 + 0.13709901006661042888e-1 * t99460 + 0.15421710918628844643e0 * t106446 - 0.86736281882051994623e-1 * t106448 - 0.28912093960683998208e-1 * t99481 + 0.78062653693846795158e1 * t93349 * t25392 * t113387 + 0.13010442282307799193e1 * t27353 * t25392 * t76161 + 0.28912093960683998208e-1 * t99496 + 0.13010442282307799193e1 * t27199 * t29691 + 0.26020884564615598386e1 * t27199 * t29669 + 0.13010442282307799193e1 * t7070 * t7076 * t29636 * t1558 * t231 - 0.78062653693846795158e1 * t7070 * t25317 * t7759 * t6048 - 0.19514881078765566038e-2 * t99520 - 0.51405703062096148812e-1 * t99522 - 0.19756347548806534796e1 * t27189 * t6072;
    (t113412,)
}
