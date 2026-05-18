//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1053/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1053<F: Float>(t1733: F, t6470: F, t3384: F, t1732: F, t20644: F, t3433: F, t17092: F, t6439: F, t6438: F, t1150: F, t12256: F, t22688: F) -> (F, F, F, F, F, F) {
    let t24212 = t1733 * t6470;
    let t24214 = F::new(6.0) * t3384 * t24212;
    let t24215 = t20644 * t1732;
    let t24217 = F::new(0.48245938496077605201e2) * t3433 * t24215;
    let t24219 = F::new(6.0) * t17092 * t6439;
    let t24220 = t6438 * t1732;
    let t24221 = t24220 * t1150;
    let t24223 = F::new(6.0) * t3433 * t24221;
    let t24228 = t12256 * t22688;
    (t24214, t24217, t24219, t24220, t24223, t24228)
}
