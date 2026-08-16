//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1122/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1122(t31844: f64, t8478: f64, t8479: f64, t246: f64, t826: f64, t854: f64, t2718: f64, t843: f64, t8484: f64, t839: f64, t31752: f64, t31753: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119751 = t8478 * t8479 * t31844;
    let t119752 = t826 * t246;
    let t119757 = t854 * t246;
    let t119763 = t8478 * t8484 * t2718 * t843;
    let t119764 = t119763 * t839;
    let t119765 = 0.34708173928447610098e-2_f64 * t119764;
    let t119767 = t31752 * t31753 * t854;
    (t119751, t119752, t119757, t119763, t119765, t119767)
}
