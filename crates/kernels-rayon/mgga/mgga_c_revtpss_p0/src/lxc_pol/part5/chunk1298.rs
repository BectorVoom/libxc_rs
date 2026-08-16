//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1298/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1298(t12511: f64, t17023: f64, t17026: f64, t1745: f64, t20471: f64, t20568: f64, t20571: f64, t20573: f64, t20576: f64, t20579: f64, t20582: f64, t20597: f64, t3447: f64, t435: f64, t5120: f64, t5125: f64, t5143: f64, t6487: f64, t6503: f64) -> f64 {
    let t20602 = 2.0_f64 * t17026 * t1745 + 2.0_f64 * t5120 * t5143 - 2.0_f64 * t12511 * t6487 + 1.0_f64 * t3447 * t6503 + t20471 - 0.19751673498613801407e-1_f64 * t20568 + t20571 - t20573 - t20576 + t20579 + t20582 - 0.310907e-1_f64 * t20597 * t435 - 4.0_f64 * t17023 * t5125;
    t20602
}
