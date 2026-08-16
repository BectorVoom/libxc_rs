//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1393/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1393(t107092: f64, t107131: f64, t107180: f64, t107208: f64, t1985: f64, t26193: f64, t28205: f64, t107056: f64, t1375: f64, t1807: f64, t1842: f64, t20060: f64, t2016: f64, t28107: f64, t28186: f64, t3887: f64, t539: f64, t568: f64, t74860: f64, t7729: f64, t81282: f64, t97529: f64, t97537: f64, t97548: f64) -> (f64, f64) {
    let t107210 = t107092 + t107131 + t107180 + t107208;
    let t107214 = t1985 * t26193 * t28205;
    let t107220 = 6.0_f64 * t1375 * t3887 * t28186 * t1842 + 0.23029076935875170111e0_f64 * t97529 - 0.16449340668482264365e-1_f64 * t107056 + t81282 + 6.0_f64 * t20060 * t7729 + 3.0_f64 * t1807 * t28107 * t568 + t539 * t107210 * t568 - 0.24674011002723396548e-1_f64 * t107214 + 0.11514538467937585055e0_f64 * t97537 - 0.11514538467937585055e0_f64 * t97548 - 3.0_f64 * t74860 * t2016;
    (t107210, t107220)
}
