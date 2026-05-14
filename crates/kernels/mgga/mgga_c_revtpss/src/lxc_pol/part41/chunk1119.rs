//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1119/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1119<F: Float>(t18330: F, t18343: F, t18361: F, t18405: F, t18454: F, t18489: F, t18524: F, t18654: F, t225: F, t6048: F, t886: F, t11008: F, t251: F, t5977: F, t1558: F, t1568: F) -> (F, F, F, F, F) {
    let t18657 = t18330 + t18343 + t18361 + t18405 + t18454 + t18489 + t18524 + t18654;
    let t18658 = t18657 * t225;
    let t18662 = t6048 * t886;
    let t18663 = t11008 * t18662;
    let t18677 = t251 * t5977;
    let t18681 = t1568 * t1558;
    (t18657, t18658, t18663, t18677, t18681)
}
