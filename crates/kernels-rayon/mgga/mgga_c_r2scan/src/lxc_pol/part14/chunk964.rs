//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 964/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk964(t1125: f64, t1305: f64, t1127: f64, t1312: f64, t1129: f64, t1131: f64, t1133: f64, t1135: f64, t1137: f64, t1310: f64, t3526: f64, t3530: f64, t3534: f64, t3538: f64, t3542: f64, t839: f64) -> (f64, f64, f64) {
    let t11244 = t1125 * t1305;
    let t11249 = t1312 * t1127;
    let t11273 = -0.9214113627294e1_f64 * t11249 - 0.18428227254588e2_f64 * t3526 * t839 - 0.9214113627294e1_f64 * t1129 * t1310 + 0.734774460522e2_f64 * t3530 * t839 + 0.367387230261e2_f64 * t1131 * t1310 - 0.7662840944824e2_f64 * t3534 * t839 - 0.3831420472412e2_f64 * t1133 * t1310 + 0.3101306810232e2_f64 * t3538 * t839 + 0.1550653405116e2_f64 * t1135 * t1310 - 0.4355305902528e1_f64 * t3542 * t839 - 0.2177652951264e1_f64 * t1137 * t1310 + 0.734774460522e2_f64 * t1129 * t1312;
    (t11244, t11249, t11273)
}
