//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1195/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1195(t32374: f64, t4292: f64, t572: f64, t26123: f64, t7741: f64, t28042: f64, t7330: f64, t1459: f64, t34004: f64, t2040: f64, t28271: f64, t127439: f64, t127442: f64, t127443: f64, t127447: f64, t127449: f64, t127453: f64, t127455: f64, t127459: f64, t1461: f64, t1918: f64, t32354: f64, t32377: f64, t33992: f64, t5802: f64, t8607: f64) -> f64 {
    let t127462 = 6.0_f64 * t572 * t32374 * t4292;
    let t127465 = 12.0_f64 * t572 * t26123 * t7741;
    let t127468 = 12.0_f64 * t572 * t7330 * t28042;
    let t127472 = 6.0_f64 * t1459 * t34004;
    let t127475 = t2040 * t28271;
    let t127477 = 3.0_f64 * t1461 * t33992 + 3.0_f64 * t1918 * t32354 + 6.0_f64 * t5802 * t8607 + 6.0_f64 * t127439 + t127442 + 12.0_f64 * t127443 + t127447 + t127449 + t127453 + t127455 + t127459 + t127462 + t127465 + t127468 + t127472 + 12.0_f64 * t127475 + t32377;
    t127477
}
