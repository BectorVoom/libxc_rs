//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1279/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1279<F: Float>(t1214: F, t5341: F, t5332: F, t3720: F, t1250: F, t5346: F, t16725: F, t5312: F, t16729: F, t1222: F, t12855: F, t12910: F, t13069: F, t17437: F, t17438: F, t17444: F, t17447: F, t17448: F, t17453: F, t1797: F, t3631: F, t3674: F) -> (F, F) {
    let t17454 = t5341 * t1214;
    let t17455 = t5332 * t17454;
    let t17456 = t3720 * t17455;
    let t17459 = t1250 * t1214;
    let t17460 = t5346 * t17459;
    let t17461 = t3720 * t17460;
    let t17464 = t5312 * t16725;
    let t17467 = t5312 * t16729;
    let t17470 = -t17437 - 0.22866142996303859718e-2 * t17438 * t3674 + 0.21437009059034868486e-3 * t13069 * t1797 + t17444 - t17447 - 0.28582678745379824648e-3 * t17448 * t3631 - t17453 - 0.85748036236139473944e-3 * t12855 * t17456 + 0.85748036236139473944e-3 * t12910 * t17461 + t1222 * t17464 / 108.0 + t1222 * t17467 / 216.0;
    (t17454, t17470)
}
