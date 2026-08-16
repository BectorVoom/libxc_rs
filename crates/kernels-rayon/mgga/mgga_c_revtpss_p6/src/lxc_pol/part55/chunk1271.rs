//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1271/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1271(t111371: f64, t1936: f64, t572: f64, t101705: f64, t7547: f64, t7953: f64, t1916: f64, t32773: f64, t7331: f64, t8118: f64, t28042: f64, t7553: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t129029 = 6.0_f64 * t572 * t111371 * t1936;
    let t129032 = 6.0_f64 * t572 * t101705 * t1936;
    let t129034 = 3.0_f64 * t7547 * t7953;
    let t129039 = 6.0_f64 * t1916 * t32773;
    let t129045 = 6.0_f64 * t8118 * t7331;
    let t129048 = 6.0_f64 * t572 * t7553 * t28042;
    (t129029, t129032, t129034, t129039, t129045, t129048)
}
