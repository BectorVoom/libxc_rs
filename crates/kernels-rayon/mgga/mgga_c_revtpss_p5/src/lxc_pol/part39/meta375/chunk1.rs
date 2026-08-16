//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1328/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1328(t3151: f64, t357: f64, t15907: f64, t3117: f64, t11883: f64, t11888: f64, t16037: f64, t16040: f64, t16045: f64, t16049: f64, t16052: f64, t16057: f64, t16062: f64, t16064: f64, t16067: f64, t1656: f64, t3115: f64, t3241: f64, t4887: f64, t4896: f64, t4902: f64) -> f64 {
    let t16068 = t3151 * t357;
    let t16069 = t15907 * t16068;
    let t16070 = t3117 * t16069;
    let t16073 = -t16037 + 0.14291339372689912324e-3_f64 * t11888 - 0.42874018118069736972e-3_f64 * t3115 * t16040 - 0.21437009059034868486e-3_f64 * t3115 * t16045 + 0.22866142996303859718e-2_f64 * t16049 * t4902 - 0.45732285992607719436e-2_f64 * t16052 * t4896 + t16057 + 11.0_f64 / 324.0_f64 * t11883 * t1656 + t16062 - t16064 - t3241 * t4887 / 54.0_f64 + 0.21437009059034868486e-3_f64 * t16067 * t16070;
    t16073
}
