//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 825/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk825(t1165: f64, t6841: f64, t7351: f64, t2068: f64, t7379: f64, t8658: f64, t8666: f64, t9226: f64, t9228: f64, t9239: f64, t9248: f64, t9249: f64, t9250: f64, t9252: f64, t9254: f64, t9261: f64, t9263: f64, t9264: f64, t9265: f64, t9584: f64, t9590: f64, t9594: f64, t9598: f64) -> (f64, f64) {
    let t9601 = t1165 * t7351 * t6841;
    let t9602 = t2068 * t9601;
    let t9606 = t7379 + t9226 + t9228 + 0.62896184579208304136e-3_f64 * t9584 + 0.10718504529517434243e-2_f64 * t9590 + 0.42874018118069736972e-3_f64 * t9594 - 0.15724046144802076034e-2_f64 * t9598 - 0.94344276868812456204e-3_f64 * t9602 + t9239 - 0.94344276868812456204e-3_f64 * t8658 + 0.20965394859736101378e-3_f64 * t8666 + t9248 + t9249 + t9250 - t9252 + t9254 - t9261 + t9263 + t9264 - t9265;
    (t9601, t9606)
}
