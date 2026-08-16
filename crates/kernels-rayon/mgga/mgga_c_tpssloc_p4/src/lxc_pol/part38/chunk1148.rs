//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1148/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1148(t1557: f64, t2793: f64, t2842: f64, t4434: f64, t931: f64, t10740: f64, t10765: f64, t14376: f64, t14378: f64, t14381: f64, t14384: f64, t14387: f64, t14391: f64, t14394: f64, t14398: f64, t14419: f64, t2861: f64, t311: f64, t4416: f64, t4438: f64) -> (f64, f64) {
    let t14422 = t1557 * t2793;
    let t14424 = 6.0_f64 * t2842 * t14422;
    let t14425 = t4434 * t931;
    let t14428 = t14376 - t14378 + t14381 + t14384 + t14387 - t14391 - t14394 - t14398 - 4.0_f64 * t10740 * t4416 + 0.64327917994770140268e2_f64 * t10765 * t4438 - 0.310907e-1_f64 * t14419 * t311 - t14424 - 4.0_f64 * t2861 * t14425;
    (t14424, t14428)
}
