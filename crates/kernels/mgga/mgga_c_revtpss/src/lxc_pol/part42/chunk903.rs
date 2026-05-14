//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 903/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk903<F: Float>(t30: F, t1312: F, t1518: F, t4248: F, t5877: F, t5883: F, t5920: F, t93: F, t5545: F, t5547: F, t5570: F, t5572: F, t1907: F, t1468: F, t3833: F, t513: F, t5824: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t6773 = 2.0 * t1312 * t5920 + 4.0 * t1518 * t4248 + 2.0 * t5883 * t93 + t5877;
    let t6777 = 8.0 * t5545;
    let t6778 = 8.0 * t5547;
    let t6779 = 2.0 * t5570;
    let t6780 = 0.11696447245269292414e1 * t5572;
    let t6781 = t1907 * t1907;
    let t6785 = t1468 * t1468;
    let t6791 = piecewise3(t31, 0.0, 4.0 / 9.0 * t3833 * t6785 + 4.0 / 3.0 * t513 * t5824);
    (t6773, t6777, t6778, t6779, t6780, t6781, t6785, t6791)
}
