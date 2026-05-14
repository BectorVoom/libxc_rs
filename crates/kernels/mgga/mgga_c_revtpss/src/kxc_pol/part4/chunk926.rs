//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 926/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk926<F: Float>(t4067: F, t786: F, t1364: F, t213: F, t4066: F, t1420: F, t1426: F, t3917: F, t64: F, t843: F, t112: F, t2289: F, t666: F, t2341: F, t625: F, t2367: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10168 = t786 * t4067;
    let t10169 = t10168 * t1364;
    let t10171 = t213 * t4066;
    let t10174 = t1420 * t1426;
    let t10175 = t786 * t10174;
    let t10176 = t10175 * t3917;
    let t10199 = t64 * t843;
    let t10201 = 154.0 / 27.0 * t10199 * t112;
    let t10202 = t2289 * t666;
    let t10204 = t625 * t2341;
    let t10206 = t625 * t2367;
    (t10169, t10171, t10175, t10176, t10199, t10201, t10202, t10204, t10206)
}
