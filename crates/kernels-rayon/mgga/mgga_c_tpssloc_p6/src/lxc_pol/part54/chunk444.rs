//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 444/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk444(t2085: f64, t539: f64, t553: f64, t2011: f64, t544: f64) -> (f64, f64, f64) {
    let t2086 = t539 * t2085;
    let t2089 = t553 * t2085;
    let t2091 = 0.16449340668482264365e-1_f64 * t2011 + t544 * t2089;
    (t2086, t2089, t2091)
}
