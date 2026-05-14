//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1187/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1187<F: Float>(t5063: F, t5105: F, t12361: F, t6439: F, t3379: F, t6471: F, t12429: F, t12470: F, t17032: F, t20606: F, t20609: F, t20612: F, t20615: F, t20619: F, t20622: F, t20626: F, t20631: F, t20633: F, t3452: F, t3477: F, t5147: F) -> (F, F, F, F) {
    let t20635 = 2.0 * t5063 * t5105;
    let t20637 = 2.0 * t12361 * t6439;
    let t20639 = 1.0 * t3379 * t6471;
    let t20640 = 0.64327917994770140268e2 * t17032 * t5147 + 6.0 * t3477 * t20606 - 4.0 * t3452 * t20609 - 0.19298375398431042081e3 * t12429 * t20612 - 2.0 * t3452 * t20615 + 0.32163958997385070134e2 * t3477 * t20619 + 0.64327917994770140268e2 * t3477 * t20622 + 0.2069040516770936012e4 * t12470 * t20626 - t20631 - t20633 - t20635 + t20637 - t20639;
    (t20635, t20637, t20639, t20640)
}
