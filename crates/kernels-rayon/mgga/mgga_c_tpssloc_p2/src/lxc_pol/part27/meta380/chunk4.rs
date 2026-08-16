//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1560/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1560(t13550: f64, t13563: f64, t10296: f64, t10298: f64, t10302: f64, t13566: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64) -> (f64, f64, f64) {
    let t14287 = 0.27785333333333333334e0_f64 * t13550;
    let t14291 = 0.22954444444444444444e0_f64 * t13563;
    let t14304 = -0.68863333333333333333e0_f64 * t13566 - 0.57386111111111111112e0_f64 * t13569 + 0.20659e1_f64 * t13572 - 0.68863333333333333334e0_f64 * t13575 - 0.34431666666666666667e0_f64 * t13578 - 0.309885e1_f64 * t13581 + 0.20659e1_f64 * t13584 + 0.103295e1_f64 * t13587 - 0.23154444444444444444e0_f64 * t10296 + 0.69463333333333333333e-1_f64 * t10302 + 0.23154444444444444444e-1_f64 * t10298;
    (t14287, t14291, t14304)
}
