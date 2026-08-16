//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1314/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1314(t1558: f64, t231: f64, t6048: f64, t106446: f64, t106448: f64, t25317: f64, t25392: f64, t27189: f64, t27199: f64, t27353: f64, t29636: f64, t29669: f64, t29691: f64, t6072: f64, t7070: f64, t7076: f64, t76161: f64, t7759: f64, t93349: f64, t99456: f64, t99460: f64, t99481: f64, t99496: f64, t99520: f64, t99522: f64) -> f64 {
    let t113387 = t6048 * t1558 * t231;
    let t113412 = -0.28912093960683998208e-1_f64 * t99456 + 0.13709901006661042888e-1_f64 * t99460 + 0.15421710918628844643e0_f64 * t106446 - 0.86736281882051994623e-1_f64 * t106448 - 0.28912093960683998208e-1_f64 * t99481 + 0.78062653693846795158e1_f64 * t93349 * t25392 * t113387 + 0.13010442282307799193e1_f64 * t27353 * t25392 * t76161 + 0.28912093960683998208e-1_f64 * t99496 + 0.13010442282307799193e1_f64 * t27199 * t29691 + 0.26020884564615598386e1_f64 * t27199 * t29669 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t29636 * t1558 * t231 - 0.78062653693846795158e1_f64 * t7070 * t25317 * t7759 * t6048 - 0.19514881078765566038e-2_f64 * t99520 - 0.51405703062096148812e-1_f64 * t99522 - 0.19756347548806534796e1_f64 * t27189 * t6072;
    t113412
}
