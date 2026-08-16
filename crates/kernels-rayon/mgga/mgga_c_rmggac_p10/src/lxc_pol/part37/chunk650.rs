//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 650/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk650(t117: f64, t1540: f64, t321: f64, t325: f64, t446: f64, t618: f64, t622: f64, t1343: f64, t7321: f64, t7334: f64, t7552: f64, t7203: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30221 = t1540 * t117;
    let t30526 = t321 * t325;
    let t31817 = t446 * t618;
    let t33235 = t622 * t321;
    let t34683 = t7321 * t1343;
    let t34709 = t7334 * t7552;
    let t34735 = t892 * t7203;
    (t30221, t30526, t31817, t33235, t34683, t34709, t34735)
}
