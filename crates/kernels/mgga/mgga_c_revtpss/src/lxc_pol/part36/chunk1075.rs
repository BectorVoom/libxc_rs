//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1075/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1075<F: Float>(t24428: F, t24470: F, t300: F, t20895: F, t5184: F, t1196: F, t24214: F, t24217: F, t24219: F, t24223: F, t24255: F, t24257: F, t24259: F, t24261: F, t24264: F, t24326: F, t24329: F) -> (F, F, F) {
    let t24472 = t300 * (t24428 + t24470);
    let t24473 = t20895 * t5184;
    let t24475 = F::new(0.51947577317044391277e2) * t1196 * t24473;
    let t24476 = -t24214 + t24217 - t24219 + t24223 + t24255 + t24257 + t24259 + t24261 - t24264 + t24326 + t24329 + t24472 - t24475;
    (t24472, t24475, t24476)
}
