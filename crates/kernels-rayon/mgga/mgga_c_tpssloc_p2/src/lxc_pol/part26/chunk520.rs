//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 520/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk520(t259: f64, t2592: f64, t2594: f64, t2597: f64, t2711: f64, t2713: f64, t2720: f64, t2743: f64, t855: f64, t866: f64) -> f64 {
    let t2745 = t259 * t2592 + 2.0_f64 * t259 * t2594 + t259 * t2711 - 2.0_f64 * t2597 * t866 - 2.0_f64 * t2713 * t866 + 2.0_f64 * t2720 * t855 - t2743 * t855;
    t2745
}
