//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1397/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1397(t33761: f64, t580: f64, t1851: f64, t8702: f64, t33783: f64, t576: f64, t1858: f64, t8692: f64, t2029: f64, t8110: f64, t2022: f64, t8119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123326 = t33761 * t580;
    let t123330 = t1851 * t8702;
    let t123331 = t576 * t33783;
    let t123332 = t8692 * t1858;
    let t123334 = t8110 * t2029;
    let t123335 = t2022 * t8119;
    (t123326, t123330, t123331, t123332, t123334, t123335)
}
