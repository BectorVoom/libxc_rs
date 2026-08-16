//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1375/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1375(t3209: f64, t3213: f64, t3215: f64, t193: f64, t3216: f64, t336: f64, t41992: f64, t41998: f64, t42002: f64, t42005: f64, t42025: f64, t42031: f64, t42097: f64, t42105: f64, t42682: f64, t42686: f64, t42688: f64) -> f64 {
    let t43629 = t3209 * t3209;
    let t43634 = t3213 * t3213;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0_f64 / t43636;
    let t43641 = -3.0_f64 * t193 * t3216 * t336 * t43629 - 6.0_f64 * t193 * t336 * t43634 * t43637 + t41992 - t41998 - t42002 + t42005 + t42025 - t42031 + t42097 + t42105 - t42682 + t42686 - t42688;
    t43641
}
