//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1396/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1396(t1737: f64, t3451: f64, t1160: f64, t5117: f64, t1170: f64, t12511: f64, t12553: f64, t16809: f64, t16832: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t16998: f64, t17020: f64, t3454: f64, t435: f64, t5125: f64) -> f64 {
    let t17023 = t1737 * t3451;
    let t17026 = t5117 * t1160;
    let t17029 = 0.10254018858216406658e4_f64 * t12553 * t16998 - 4.0_f64 * t12511 * t5125 + t16809 - 0.19751673498613801407e-1_f64 * t16832 - 0.310907e-1_f64 * t17020 * t435 - t16837 - t16839 - t16842 - t16844 - t16846 - t16945 - 2.0_f64 * t17023 * t3454 + 2.0_f64 * t17026 * t1170;
    t17029
}
