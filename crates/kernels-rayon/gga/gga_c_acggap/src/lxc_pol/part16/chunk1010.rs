//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1010/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1010(t35646: f64, t171: f64, t5011: f64, t2310: f64, t7780: f64, t31643: f64, t527: f64, t1418: f64, t7605: f64, t1347: f64, t1980: f64, t35383: f64, t7458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35647 = 0.1528125e-1_f64 * t35646;
    let t35649 = t171 * t5011;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35672 = t7605 * t1418;
    let t35673 = 0.68598428988911579156e-2_f64 * t35672;
    let t35678 = t7605 * t1347;
    let t35679 = 0.68598428988911579156e-2_f64 * t35678;
    let t35682 = t1980 * t7458 * t35383;
    (t35647, t35649, t35662, t35664, t35673, t35679, t35682)
}
