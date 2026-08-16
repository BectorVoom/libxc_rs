//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 766/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk766(t123: f64, t7275: f64, t734: f64, t1858: f64, t3294: f64, t321: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t7276 = t7275 * t123;
    let t7277 = t7276 * t734;
    let t7280 = t1858 * t3294;
    let t7281 = t7280 * t734;
    let t7284 = t321 * t935;
    (t7276, t7277, t7280, t7281, t7284)
}
