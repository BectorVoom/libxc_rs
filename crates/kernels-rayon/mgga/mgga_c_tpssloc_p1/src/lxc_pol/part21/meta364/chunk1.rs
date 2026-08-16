//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1793/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1793(t13520: f64, t2845: f64, t10650: f64, t1557: f64, t2787: f64, t4396: f64, t2770: f64, t3966: f64, t607: f64) -> (f64, f64, f64, f64) {
    let t13522 = 0.16081979498692535067e2_f64 * t13520 * t2845;
    let t13524 = 1.0_f64 * t10650 * t1557;
    let t13526 = 2.0_f64 * t2787 * t4396;
    let t13527 = t2770 * t3966;
    let t13528 = t13527 * t607;
    (t13522, t13524, t13526, t13528)
}
