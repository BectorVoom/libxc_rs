//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1339/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1339(t1981: f64, t4573: f64, t68: f64, t1289: f64, t578: f64, t1792: f64, t18649: f64, t21123: f64, t21129: f64, t21133: f64, t5489: f64, t5785: f64, t5794: f64, t69135: f64, t69139: f64, t69228: f64, t69232: f64, t69242: f64, t69245: f64, t69248: f64, t69251: f64) -> f64 {
    let t71447 = t1981 * t4573 * t68;
    let t71451 = t578 * t1289 * t68;
    let t71460 = -4.0_f64 / 3.0_f64 * t21123 * t5794 - 10.0_f64 / 3.0_f64 * t18649 * t21129 - 10.0_f64 / 3.0_f64 * t5785 * t69135 - 10.0_f64 / 3.0_f64 * t5785 * t69139 - 5.0_f64 / 3.0_f64 * t18649 * t21133 - 5.0_f64 / 3.0_f64 * t5785 * t69228 - 5.0_f64 / 3.0_f64 * t5785 * t69232 + 10.0_f64 / 3.0_f64 * t71447 * t5489 - 4.0_f64 / 3.0_f64 * t71451 * t69242 - 2.0_f64 / 3.0_f64 * t69245 * t1792 - 2.0_f64 / 3.0_f64 * t69248 * t1792 - 2.0_f64 / 3.0_f64 * t69251 * t1792;
    t71460
}
