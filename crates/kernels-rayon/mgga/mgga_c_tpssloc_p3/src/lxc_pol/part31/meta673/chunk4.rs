//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2028/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2028(t102587: f64, t1336: f64, t1825: f64, t19654: f64, t19732: f64, t24116: f64, t27075: f64, t27086: f64, t27098: f64, t29343: f64, t29349: f64, t3777: f64, t5234: f64, t5250: f64, t5334: f64, t6415: f64, t6420: f64, t7208: f64, t84595: f64, t84597: f64, t91018: f64, t91043: f64, t91045: f64, t93607: f64, t97179: f64, t97200: f64) -> f64 {
    let t102790 = -2.0_f64 * t5234 * t27098 - t84595 + 2.0_f64 * t5334 * t102587 * t5250 - t91018 + t84597 - 2.0_f64 * t1336 * t93607 * t1825 - 2.0_f64 * t3777 * t29349 - t1336 * t24116 * t6415 - t3777 * t29343 - t1336 * t24116 * t6420 - 0.23029076935875170111e0_f64 * t97179 + 4.0_f64 * t19654 * t27075 - t91043 + t91045 - t1336 * t7208 * t19732 - 2.0_f64 * t5234 * t27086 - 0.38381794893125283518e-1_f64 * t97200;
    t102790
}
