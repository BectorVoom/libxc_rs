//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2932/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2932(t15817: f64, t3173: f64, t16158: f64, t3188: f64, t1063: f64, t15193: f64, t247: f64, t3109: f64, t11233: f64, t11656: f64, t12026: f64, t15707: f64, t15791: f64, t15830: f64, t15834: f64, t15952: f64, t3106: f64, t3177: f64, t3184: f64, t42391: f64, t4825: f64, t4834: f64) -> f64 {
    let t53353 = t15817 * t3173;
    let t53359 = t3188 * t16158;
    let t53363 = t1063 * t247 * t3109 * t15193;
    let t53377 = 0.85748036236139473944e-3_f64 * t53353 - 0.22866142996303859718e-2_f64 * t15830 * t3177 - 0.3811023832717309953e-2_f64 * t15830 * t3184 + 0.57165357490759649295e-3_f64 * t53359 + 0.28582678745379824648e-3_f64 * t53363 - 0.42874018118069736972e-3_f64 * t42391 * t4825 + 0.91464571985215438873e-2_f64 * t3106 * t15791 - 0.76220476654346199061e-2_f64 * t3106 * t15834 - 0.42874018118069736972e-3_f64 * t15707 * t12026 + 0.45732285992607719436e-2_f64 * t11656 * t15952 - 0.85748036236139473944e-3_f64 * t4834 * t11233;
    t53377
}
