//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2842/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2842(t231: f64, t23244: f64, t243: f64, t2661: f64, t2662: f64, t10871: f64, t40693: f64, t76569: f64, t23263: f64, t40864: f64, t23114: f64, t40462: f64, t40810: f64, t51042: f64, t51055: f64, t62108: f64, t62111: f64, t62114: f64, t62129: f64, t62135: f64, t62148: f64, t76804: f64, t76808: f64, t76812: f64, t76814: f64, t76818: f64, t775: f64, t828: f64, t851: f64) -> f64 {
    let t76823 = t2661 * t2662 * t243 * t23244 * t231;
    let t76827 = t2661 * t40693 * t76569 * t10871;
    let t76835 = t40864 * t23263;
    let t76843 = 0.24009450146119052705e-1_f64 * t62108 + 0.12004725073059526352e0_f64 * t76804 - 0.15246000842785598467e-2_f64 * t76808 - 0.42874018118069736973e-3_f64 * t76812 + 0.40015750243531754507e-2_f64 * t76814 + 0.71456696863449561619e-5_f64 * t76818 + 0.71456696863449561619e-5_f64 * t76823 + 0.42874018118069736973e-4_f64 * t76827 + 0.18007087609589289528e0_f64 * t851 * t40462 * t828 * t23114 * t775 + 0.1084295579938911763e-3_f64 * t62111 + 7.0_f64 / 12.0_f64 * t76835 + 0.18007087609589289529e-1_f64 * t62114 + t40810 - 0.1372140075850703862e-3_f64 * t51042 + 0.45732285992607719437e-2_f64 * t62129 + 7.0_f64 / 48.0_f64 * t62135 - 0.38115002106963996168e-4_f64 * t62148 - 0.5421477899694558815e-4_f64 * t51055;
    t76843
}
