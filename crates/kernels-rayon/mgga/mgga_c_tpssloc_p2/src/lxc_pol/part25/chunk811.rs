//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 811/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk811(t10523: f64, t10524: f64, t2932: f64, t959: f64, t10195: f64, t2768: f64, t123: f64) -> (f64, f64) {
    let t10526 = t10523 * t10524 * t2932;
    let t10528 = 0.10389515463408878255e3_f64 * t959 * t10526;
    let t10529 = t2768 * t10195;
    let t10530 = t123 * t10529;
    (t10528, t10530)
}
