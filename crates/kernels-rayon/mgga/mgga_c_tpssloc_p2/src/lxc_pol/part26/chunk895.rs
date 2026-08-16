//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 895/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk895(t10544: f64, t2798: f64, t2807: f64, t896: f64, t2815: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t10307: f64, t10314: f64, t10320: f64, t10323: f64, t10530: f64, t10538: f64, t10542: f64) -> (f64, f64, f64) {
    let t10545 = 0.93932222222222222223e0_f64 * t10544;
    let t10547 = t2798 * t896 * t2807;
    let t10550 = t2815 * t896 * t2807;
    let t10553 = -0.60384999999999999999e0_f64 * t10530 - 0.27595e0_f64 * t10296 + 0.16557e0_f64 * t10302 + 0.5519e-1_f64 * t10298 - 0.36793333333333333333e-1_f64 * t10307 - 0.82785e-1_f64 * t10323 + 0.181155e1_f64 * t10538 - 0.82785e-1_f64 * t10314 + 0.49671e0_f64 * t10320 - t10542 - t10545 - 0.3883875e1_f64 * t10547 + 0.247573125e0_f64 * t10550 - 0.33114e0_f64 * t10300;
    (t10547, t10550, t10553)
}
