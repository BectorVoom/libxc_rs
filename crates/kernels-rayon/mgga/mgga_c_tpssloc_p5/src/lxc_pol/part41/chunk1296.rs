//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1296/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1296(t29895: f64, t30514: f64, t30521: f64, t626: f64, t111125: f64, t111127: f64, t111129: f64, t1449: f64, t26129: f64, t29903: f64, t30063: f64, t30284: f64, t30293: f64, t30297: f64, t4067: f64, t5464: f64, t5484: f64, t662: f64, t666: f64, t8128: f64, t8137: f64, t8180: f64, t8184: f64, t96718: f64) -> f64 {
    let t111775 = t29895 * t30514;
    let t111803 = t626 * t30521;
    let t111805 = -110.0_f64 / 27.0_f64 * t111125 - 10.0_f64 / 9.0_f64 * t111127 - 20.0_f64 / 9.0_f64 * t111775 + 5.0_f64 / 12.0_f64 * t8128 * t8184 * t5484 * t666 - 5.0_f64 / 36.0_f64 * t8137 * t30063 * t5484 * t662 - 5.0_f64 / 4.0_f64 * t29903 * t8184 * t5464 * t662 - 3.0_f64 / 2.0_f64 * t29903 * t8180 * t96718 + 5.0_f64 / 2.0_f64 * t29903 * t30293 * t26129 - 25.0_f64 / 18.0_f64 * t8128 * t30297 * t30284 + 5.0_f64 / 6.0_f64 * t8128 * t8184 * t4067 * t1449 + 110.0_f64 / 27.0_f64 * t111129 + 40.0_f64 / 27.0_f64 * t111803;
    t111805
}
