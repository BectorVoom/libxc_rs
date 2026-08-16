//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1308/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1308(t3941: f64, t7769: f64, t1401: f64, t7467: f64, t1458: f64, t577: f64, t7010: f64, t7758: f64, t7768: f64, t2018: f64, t3701: f64) -> (f64, f64) {
    let t7771 = 27.0_f64 * t3941 * t7769;
    let t7773 = 0.135e2_f64 * t1401 * t7467;
    let t7774 = 0.45e1_f64 * t7758 * t577 + 0.135e2_f64 * t7010 * t1458 + t7768 + t7771 + t7773;
    let t8643 = t3701 * t2018;
    (t7774, t8643)
}
