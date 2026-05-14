//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1037/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1037<F: Float>(t3172: F, t5298: F, t3711: F, t5278: F, t5269: F, t1261: F, t12256: F, t13099: F, t1224: F, t140: F, t5052: F, t1222: F, t3636: F, t5391: F, t5381: F, t1803: F, t3666: F) -> (F, F, F, F, F, F, F, F) {
    let t17209 = t3172 * t5298;
    let t17211 = 0.19055119163586549765e-3 * t3711 * t17209;
    let t17217 = t3172 * t5278;
    let t17219 = 0.19055119163586549765e-3 * t3711 * t17217;
    let t17225 = t3172 * t5269;
    let t17227 = 0.3811023832717309953e-3 * t1261 * t17225;
    let t17235 = t13099 * t12256;
    let t17240 = t140 * t1224;
    let t17241 = t17240 * t5052;
    let t17243 = t1222 * t17241 / 216.0;
    let t17258 = 0.10162730220579493208e-2 * t5391 * t3636;
    let t17260 = 0.19055119163586549765e-3 * t5381 * t3636;
    let t17283 = t3666 * t1803;
    (t17211, t17219, t17227, t17235, t17243, t17258, t17260, t17283)
}
