//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1170/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1170(t120975: f64, t1885: f64, t121034: f64, t1390: f64, t32192: f64, t5727: f64, t828: f64, t8583: f64, t5629: f64, t8589: f64, t121354: f64, t33969: f64, t8591: f64) -> (f64, f64, f64, f64) {
    let t125706 = t120975 * t1885;
    let t125717 = t8583 * t121034 * t32192 * t1390 * t828 * t5727;
    let t125721 = t8583 * t8589 * t32192 * t5629;
    let t125732 = t8591 * t121354 * t33969;
    (t125706, t125717, t125721, t125732)
}
