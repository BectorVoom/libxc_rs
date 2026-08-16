//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1071/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1071(t5: f64, t2035: f64, t34399: f64, t7935: f64, t8764: f64, t13272: f64, t8736: f64, t8142: f64, t8435: f64, t2247: f64, t32798: f64, t32802: f64, t33621: f64, t34173: f64, t34177: f64, t34181: f64, t8623: f64, t8737: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t34400 = t34399 * t2035;
    let t34401 = t8764 * t7935;
    let t34402 = t13272 * t8736;
    let t34409 = t8435 * t8142;
    let t34410 = t2247 * t34409;
    let t34418 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t34402 * t8623 - 5.0_f64 / 24.0_f64 * t32798 * t34173 - 5.0_f64 / 36.0_f64 * t32802 * t34177 + 5.0_f64 / 144.0_f64 * t34410 * t8623 + 5.0_f64 / 72.0_f64 * t8737 * t34181 + 5.0_f64 / 144.0_f64 * t8737 * t33621);
    (t34400, t34401, t34402, t34409, t34410, t34418)
}
