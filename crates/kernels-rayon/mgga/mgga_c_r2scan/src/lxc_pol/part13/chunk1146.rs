//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1146/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1146(t37848: f64, t37851: f64, t37823: f64, t37834: f64, t37835: f64, t37838: f64, t37843: f64, t39738: f64, t39740: f64, t39742: f64, t39746: f64, t39749: f64) -> f64 {
    let t39752 = 0.84755945902752848174e0_f64 * t37848;
    let t39753 = 0.25426783770825854452e1_f64 * t37851;
    let t39754 = t37823 + t37834 + 0.58544643236296698112e-1_f64 * t37835 + 0.45022119329691164872e0_f64 * t37838 + t39738 - 0.86682217400542685632e-1_f64 * t39740 - 0.43341108700271342816e-1_f64 * t39742 - 0.2600466522016280569e0_f64 * t39746 + 0.13099107994629972538e-1_f64 * t39749 + 0.27439371595564631661e-2_f64 * t37843 - t39752 - t39753;
    t39754
}
