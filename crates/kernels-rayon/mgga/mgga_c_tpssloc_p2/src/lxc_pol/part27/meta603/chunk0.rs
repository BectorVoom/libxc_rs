//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2073/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2073(t23610: f64, t23665: f64, t3032: f64, t3131: f64, t23614: f64, t82431: f64, t23384: f64, t23693: f64, t23698: f64, t3166: f64, t362: f64, t23383: f64, t6712: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82539 = t23665 * t23610;
    let t82542 = t3032 * t3131;
    let t82555 = t82431 * t23614;
    let t82562 = t23384 * t23693;
    let t82564 = t23384 * t23698;
    let t82566 = t362 * t3166;
    let t82573 = t6712 * t23383;
    (t82539, t82542, t82555, t82562, t82564, t82566, t82573)
}
