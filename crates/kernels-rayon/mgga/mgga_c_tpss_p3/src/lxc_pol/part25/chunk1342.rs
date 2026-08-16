//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1342/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1342(t21139: f64, t5791: f64, t20275: f64, t6080: f64, t1675: f64, t21165: f64, t5790: f64, t21146: f64, t6073: f64, t1791: f64, t21756: f64, t5483: f64, t5785: f64, t67474: f64, t67480: f64, t67491: f64, t67496: f64, t69338: f64, t69355: f64) -> f64 {
    let t71503 = t21139 * t5791;
    let t71505 = t6080 * t20275;
    let t71508 = t1675 * t5790 * t21165;
    let t71510 = t21146 * t5791;
    let t71512 = t6073 * t20275;
    let t71520 = -5.0_f64 / 3.0_f64 * t5785 * t69355 + 16.0_f64 / 9.0_f64 * t71503 + 32.0_f64 / 9.0_f64 * t71505 - t67474 - 8.0_f64 / 9.0_f64 * t71508 - 8.0_f64 / 9.0_f64 * t71510 - 16.0_f64 / 9.0_f64 * t71512 + t67480 + t67491 + 176.0_f64 / 27.0_f64 * t67496 + t5483 * t21756 / 3.0_f64 + t1675 * t1791 * t69338 / 3.0_f64;
    t71520
}
