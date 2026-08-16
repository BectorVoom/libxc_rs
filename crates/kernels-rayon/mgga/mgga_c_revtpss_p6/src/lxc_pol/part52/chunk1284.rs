//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1284/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1284(t1916: f64, t32773: f64, t7331: f64, t8118: f64, t28042: f64, t572: f64, t7553: f64, t28986: f64, t7002: f64, t32776: f64, t127455: f64, t127459: f64, t127462: f64, t1918: f64, t2040: f64, t2115: f64, t28246: f64, t28975: f64, t28981: f64, t28990: f64, t32755: f64) -> f64 {
    let t129039 = 6.0_f64 * t1916 * t32773;
    let t129045 = 6.0_f64 * t8118 * t7331;
    let t129048 = 6.0_f64 * t572 * t7553 * t28042;
    let t129055 = 6.0_f64 * t572 * t28986 * t7002;
    let t129057 = 6.0_f64 * t1916 * t32776;
    let t129060 = 3.0_f64 * t1918 * t32755 + 6.0_f64 * t2040 * t28975 + 6.0_f64 * t2040 * t28981 + 3.0_f64 * t2040 * t28990 + 3.0_f64 * t2115 * t28246 + t127455 + t127459 + t127462 + t129039 + t129045 + t129048 + t129055 + t129057;
    t129060
}
