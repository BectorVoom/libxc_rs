//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 418/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk418(t1425: f64, t846: f64, t1409: f64, t870: f64, t1416: f64, t1419: f64, t1422: f64, t879: f64, t882: f64, t885: f64) -> (f64, f64, f64, f64) {
    let t1427 = 1.0_f64 * t846 * t1425;
    let t1429 = -t870 - 0.17123333333333333333e-1_f64 * t1409;
    let t1436 = 0.3529725e1_f64 * t1416 - t879 - 0.516475e0_f64 * t1409 + 0.6311625e0_f64 * t1419 - t882 - 0.104195e0_f64 * t1422;
    let t1437 = t1436 * t885;
    (t1427, t1429, t1436, t1437)
}
