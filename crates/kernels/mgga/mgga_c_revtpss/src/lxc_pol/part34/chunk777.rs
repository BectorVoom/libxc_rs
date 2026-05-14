//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 777/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk777<F: Float>(t11249: F, t3154: F, t1036: F, t11244: F, t11240: F, t357: F, t246: F, t676: F, t287: F, t2922: F, t275: F, t11132: F, t240: F, t624: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11250 = t11249 * t3154;
    let t11255 = t1036 * t11244;
    let t11256 = t11240 * t11255;
    let t11257 = t11249 * t357;
    let t11262 = t246 * t676;
    let t11298 = 1.0 / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11304 = 28.0 / 27.0 * t11132;
    let t11334 = 0.93011851851851851854e0 * t11132;
    let t11335 = t624 * t240;
    let t11337 = t281 * t11335 * t283;
    (t11250, t11256, t11257, t11262, t11299, t11304, t11334, t11335, t11337)
}
