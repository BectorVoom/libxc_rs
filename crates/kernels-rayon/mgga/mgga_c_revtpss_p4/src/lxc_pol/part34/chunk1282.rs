//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1282/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1282(t113285: f64, t2723: f64, t106272: f64, t106286: f64, t106316: f64, t106318: f64, t106326: f64, t113261: f64, t1579: f64, t231: f64, t23404: f64, t25391: f64, t27357: f64, t29636: f64, t6071: f64, t7053: f64, t7070: f64, t7071: f64, t7076: f64, t7759: f64, t93272: f64, t93276: f64, t99297: f64, t99307: f64, t99313: f64) -> f64 {
    let t113295 = t113285 * t2723;
    let t113320 = -0.72280234901709995519e-3_f64 * t99297 + 0.52041769129231196772e1_f64 * t25391 * t27357 * t113295 - 0.16463622957338778996e-1_f64 * t106272 + 0.16463622957338778996e-1_f64 * t106286 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t113261 * t231 - 0.39029762157531132076e-1_f64 * t99307 - 0.34697458558045176417e-2_f64 * t99313 + 0.26020884564615598386e1_f64 * t7070 * t7071 * t7759 * t6071 + t93272 + 0.26020884564615598386e1_f64 * t7070 * t7071 * t29636 * t1579 - 0.32927245914677557992e-1_f64 * t106316 - 0.86736281882051994623e-1_f64 * t106318 + 0.58544643236296698113e-1_f64 * t106326 - t93276 + 0.39512695097613069591e1_f64 * t7053 * t23404;
    t113320
}
