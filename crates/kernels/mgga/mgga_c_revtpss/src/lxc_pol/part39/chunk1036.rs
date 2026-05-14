//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1036/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1036<F: Float>(t1222: F, t12998: F, t1226: F, t697: F, t140: F, t3688: F, t3700: F, t3367: F, t404: F, t1242: F, t3603: F, t471: F, t1032: F, t3552: F, t1246: F, t247: F, t3372: F, t3634: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12999 = t1222 * t12998;
    let t13011 = t697 * t1226;
    let t13012 = t1222 * t13011;
    let t13014 = t140 * t3688;
    let t13015 = t1222 * t13014;
    let t13017 = t140 * t3700;
    let t13018 = t1222 * t13017;
    let t13026 = 1.0 / t404 / t3367;
    let t13037 = t1242 * t1242;
    let t13038 = 1.0 / t13037;
    let t13045 = t3603 * t471;
    let t13068 = t3552 * t1032;
    let t13069 = t13068 * t1246;
    let t13085 = t247 * t3634 * t3372;
    (t12999, t13012, t13015, t13018, t13026, t13038, t13045, t13069, t13085)
}
