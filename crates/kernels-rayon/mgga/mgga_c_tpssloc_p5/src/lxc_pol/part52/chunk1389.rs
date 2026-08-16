//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1389/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1389(t26504: f64, t8690: f64, t120703: f64, t120708: f64, t120709: f64, t120711: f64, t120714: f64, t120716: f64, t120719: f64, t120721: f64, t120723: f64, t120728: f64, t120730: f64, t120732: f64) -> f64 {
    let t123235 = t8690 * t26504;
    let t123242 = 3.0_f64 * t120703 + t123235 - t120708 - 2.0_f64 * t120709 - 2.0_f64 * t120711 - 2.0_f64 * t120714 - 2.0_f64 * t120716 - t120719 - t120721 - 2.0_f64 * t120723 - t120728 - t120730 - 2.0_f64 * t120732;
    t123242
}
