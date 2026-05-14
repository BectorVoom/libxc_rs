//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 759/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk759<F: Float>(t604: F, t6847: F, t1181: F, t2068: F, t157: F, t495: F, t524: F, t599: F, t7337: F, t6841: F, t1165: F, t7351: F, t7379: F, t8658: F, t8666: F, t9226: F, t9228: F, t9239: F, t9248: F, t9249: F, t9250: F, t9252: F, t9254: F, t9261: F, t9263: F, t9264: F, t9265: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9582 = t604 * t6847;
    let t9583 = t1181 * t9582;
    let t9584 = t2068 * t9583;
    let t9587 = t495 * t524 * t157;
    let t9588 = t599 * t9587;
    let t9589 = t1181 * t9588;
    let t9590 = t7337 * t9589;
    let t9592 = t604 * t6841;
    let t9593 = t1181 * t9592;
    let t9594 = t2068 * t9593;
    let t9597 = t1165 * t604 * t9587;
    let t9598 = t7337 * t9597;
    let t9601 = t1165 * t7351 * t6841;
    let t9602 = t2068 * t9601;
    let t9606 = t7379 + t9226 + t9228 + 0.62896184579208304136e-3 * t9584 + 0.10718504529517434243e-2 * t9590 + 0.42874018118069736972e-3 * t9594 - 0.15724046144802076034e-2 * t9598 - 0.94344276868812456204e-3 * t9602 + t9239 - 0.94344276868812456204e-3 * t8658 + 0.20965394859736101378e-3 * t8666 + t9248 + t9249 + t9250 - t9252 + t9254 - t9261 + t9263 + t9264 - t9265;
    (t9582, t9583, t9587, t9588, t9589, t9592, t9593, t9597, t9601, t9606)
}
