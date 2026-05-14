//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 965/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk965<F: Float>(t12610: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F, t482: F, t371: F, t372: F, t24610: F, t5302: F) -> (F, F, F) {
    let t24633 = -t12610 + 0.19755555555555555556e-1 * t16706 + 0.9877777777777777778e-2 * t20283 - 0.29633333333333333334e-1 * t20285 - 0.14816666666666666667e-1 * t20287 + 0.16462962962962962963e-1 * t24230 - 0.59266666666666666668e-1 * t24234 - 0.29633333333333333334e-1 * t24238 + 0.88900000000000000002e-1 * t24242 + 0.88900000000000000002e-1 * t24246 + 0.14816666666666666667e-1 * t24250;
    let t24634 = t482 * t24633;
    let t24636 = t371 * t372 * t24634;
    let t24639 = t5302 * t24610;
    (t24633, t24636, t24639)
}
