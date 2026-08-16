//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2241/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2241(t6601: f64, t7623: f64, t21188: f64, t26844: f64, t104658: f64, t104680: f64, t104732: f64, t1266: f64, t17307: f64, t1808: f64, t20864: f64, t20978: f64, t21111: f64, t26873: f64, t29020: f64, t29037: f64, t5287: f64, t5304: f64, t5386: f64, t6625: f64, t7624: f64, t97149: f64) -> f64 {
    let t112179 = t6601 * t7623;
    let t112195 = t26844 * t21188;
    let t112200 = -0.85748036236139473944e-3_f64 * t97149 * t20978 + 0.1270341277572436651e-3_f64 * t104658 - 0.28582678745379824648e-3_f64 * t112179 * t1266 + 0.17149607247227894789e-2_f64 * t17307 * t7623 * t5386 - 0.1270341277572436651e-2_f64 * t7624 * t21111 - 0.57165357490759649296e-3_f64 * t104732 * t1808 + 0.42874018118069736972e-3_f64 * t26873 * t6625 + 0.95275595817932748826e-3_f64 * t7624 * t20864 + 0.95275595817932748827e-3_f64 * t29037 * t5304 + 0.57165357490759649296e-3_f64 * t112195 - 0.45732285992607719436e-2_f64 * t29020 * t5287 + 0.38110238327173099531e-3_f64 * t104680;
    t112200
}
