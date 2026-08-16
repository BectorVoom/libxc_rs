//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1305/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1305(t1063: f64, t20054: f64, t19572: f64, t4894: f64, t3117: f64, t4900: f64, t11774: f64, t15926: f64, t20040: f64, t20046: f64, t20051: f64, t3106: f64, t3188: f64, t4892: f64, t4899: f64, t4912: f64, t6323: f64, t6327: f64, t6331: f64) -> f64 {
    let t20055 = t1063 * t20054;
    let t20065 = t19572 * t4894;
    let t20066 = t3117 * t20065;
    let t20069 = t19572 * t4900;
    let t20070 = t3117 * t20069;
    let t20073 = -0.28582678745379824648e-3_f64 * t11774 * t20040 + 0.14291339372689912324e-3_f64 * t3188 * t6323 + 0.14291339372689912324e-3_f64 * t1063 * t20046 + 0.15879265969655458138e-3_f64 * t20051 + 0.95275595817932748827e-4_f64 * t20055 - 0.1270341277572436651e-2_f64 * t3106 * t6327 - 0.76220476654346199061e-3_f64 * t3106 * t6323 - 0.28582678745379824648e-3_f64 * t3188 * t6331 - 0.42874018118069736972e-3_f64 * t15926 * t4912 + 0.42874018118069736972e-3_f64 * t4892 * t20066 - 0.21437009059034868486e-3_f64 * t4899 * t20070;
    t20073
}
