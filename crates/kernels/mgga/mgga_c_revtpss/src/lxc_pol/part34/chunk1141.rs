//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1141/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1141<F: Float>(t113285: F, t2723: F, t106272: F, t106286: F, t106316: F, t106318: F, t106326: F, t113261: F, t1579: F, t231: F, t23404: F, t25391: F, t27357: F, t29636: F, t6071: F, t7053: F, t7070: F, t7071: F, t7076: F, t7759: F, t93272: F, t93276: F, t99297: F, t99307: F, t99313: F) -> (F,) {
    let t113295 = t113285 * t2723;
    let t113320 = -0.72280234901709995519e-3 * t99297 + 0.52041769129231196772e1 * t25391 * t27357 * t113295 - 0.16463622957338778996e-1 * t106272 + 0.16463622957338778996e-1 * t106286 + 0.13010442282307799193e1 * t7070 * t7076 * t113261 * t231 - 0.39029762157531132076e-1 * t99307 - 0.34697458558045176417e-2 * t99313 + 0.26020884564615598386e1 * t7070 * t7071 * t7759 * t6071 + t93272 + 0.26020884564615598386e1 * t7070 * t7071 * t29636 * t1579 - 0.32927245914677557992e-1 * t106316 - 0.86736281882051994623e-1 * t106318 + 0.58544643236296698113e-1 * t106326 - t93276 + 0.39512695097613069591e1 * t7053 * t23404;
    (t113320,)
}
