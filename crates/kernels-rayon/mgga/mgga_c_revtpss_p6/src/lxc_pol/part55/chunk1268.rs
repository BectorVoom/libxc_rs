//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1268/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1268(t28184: f64, t8698: f64, t1353: f64, t26405: f64, t28167: f64, t34301: f64, t32626: f64, t7935: f64, t102019: f64, t1937: f64, t111018: f64, t28653: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128970 = 3.0_f64 * t8698 * t28184;
    let t128974 = 6.0_f64 * t28167 * t26405 * t34301 * t1353;
    let t128975 = t32626 * t7935;
    let t128977 = 2.0_f64 * t102019 * t1937;
    let t128979 = 2.0_f64 * t111018 * t1937;
    let t128981 = 2.0_f64 * t28653 * t6993;
    (t128970, t128974, t128975, t128977, t128979, t128981)
}
