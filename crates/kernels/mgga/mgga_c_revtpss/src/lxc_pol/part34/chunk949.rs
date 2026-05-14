//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 949/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk949<F: Float>(t24232: F, t3417: F, t141: F, t1145: F, t24240: F, t24248: F, t24236: F, t12296: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F) -> (F, F, F, F, F) {
    let t24288 = t3417 * t24232;
    let t24289 = t141 * t24288;
    let t24291 = t1145 * t24240;
    let t24292 = t141 * t24291;
    let t24294 = t1145 * t24248;
    let t24295 = t141 * t24294;
    let t24297 = t3417 * t24236;
    let t24298 = t141 * t24297;
    let t24312 = -t12296 + 4.0 / 9.0 * t16706 + 2.0 / 9.0 * t20283 - 2.0 / 3.0 * t20285 - t20287 / 3.0 + 10.0 / 27.0 * t24230 - 4.0 / 3.0 * t24234 - 2.0 / 3.0 * t24238 + 2.0 * t24242 + 2.0 * t24246 + t24250 / 3.0;
    (t24289, t24292, t24295, t24298, t24312)
}
