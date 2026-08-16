//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1443/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1443(t1174: f64, t3471: f64, t698: f64, t3475: f64, t3469: f64, t3477: f64, t11504: f64, t135: f64, t43713: f64, t43717: f64, t43721: f64, t43725: f64, t43754: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43835: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44424 = t1174 * t698 * t3471;
    let t44426 = t3475 * t3475;
    let t44432 = t3469 * t3469;
    let t44439 = t1174 * t698 * t3477;
    let t44445 = t1174 * t135 * t11504;
    let t44457 = -4.0_f64 / 9.0_f64 * t43768 + 2.0_f64 * t43713 + t43754 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t43717 - 6.0_f64 * t43721 - t43759 - 4.0_f64 / 3.0_f64 * t43725 + 14.0_f64 / 81.0_f64 * t43766 + 8.0_f64 / 3.0_f64 * t43770 - 4.0_f64 / 9.0_f64 * t43773 - 8.0_f64 / 9.0_f64 * t43835;
    (t44424, t44426, t44432, t44439, t44445, t44457)
}
