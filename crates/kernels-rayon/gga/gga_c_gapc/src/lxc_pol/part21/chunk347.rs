//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 347/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk347(t128: f64, t1540: f64, t21: f64, t496: f64, t138: f64, t141: f64, t1518: f64, t1521: f64, t1524: f64, t1525: f64, t1532: f64, t1535: f64, t1545: f64, t488: f64, t499: f64, t502: f64) -> (f64, f64) {
    let t1548 = t1540 * t128;
    let t1549 = t496 * t21;
    let t1552 = 0.71188398362396778151e-1_f64 * t1518 * t141 + 0.15370222373699304374e-1_f64 * t1521 * t488 - 0.16179181445999267762e-2_f64 * t1525 * t499 + 0.28766584610986698081e-2_f64 * t1525 * t502 - 0.16179181445999267762e-3_f64 * t1524 * t138 * t1532 + 0.28766584610986698082e-3_f64 * t1535 * t502 + 0.16179181445999267762e-4_f64 * t1540 * t138 * t1545 - 0.28766584610986698082e-4_f64 * t1548 * t1549;
    (t1548, t1552)
}
