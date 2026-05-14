//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 888/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk888<F: Float>(t12367: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F, t448: F, t300: F, t1733: F, t20629: F, t5063: F, t6471: F) -> (F, F, F, F) {
    let t24252 = -t12367 + 0.12361111111111111111e-1 * t16706 + 0.61805555555555555556e-2 * t20283 - 0.18541666666666666667e-1 * t20285 - 0.92708333333333333334e-2 * t20287 + 0.10300925925925925926e-1 * t24230 - 0.37083333333333333333e-1 * t24234 - 0.18541666666666666666e-1 * t24238 + 0.55625000000000000001e-1 * t24242 + 0.55625000000000000001e-1 * t24246 + 0.92708333333333333333e-2 * t24250;
    let t24253 = t24252 * t448;
    let t24255 = 0.19751673498613801407e-1 * t300 * t24253;
    let t24257 = 3.0 * t20629 * t1733;
    let t24259 = 3.0 * t5063 * t6471;
    (t24253, t24255, t24257, t24259)
}
