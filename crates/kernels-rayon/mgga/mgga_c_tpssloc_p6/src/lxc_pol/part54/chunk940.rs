//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 940/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk940(t22893: f64, t6639: f64, t23164: f64, t6546: f64, t6551: f64) -> (f64, f64, f64) {
    let t23165 = t22893 * t6639;
    let t23166 = t23164 * t23165;
    let t23167 = 0.16449340668482264365e-1_f64 * t23166;
    let t23168 = t6546 * t6551;
    (t23166, t23167, t23168)
}
