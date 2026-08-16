//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 202/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk202(t592: f64, t14: f64, t2: f64, t21: f64, t15: f64, t583: f64) -> (f64, f64, f64, f64, f64) {
    let t593 = 2.0_f64 * t592;
    let t594 = t14 * t2;
    let t596 = 0.1356e2_f64 * t594 * t21;
    let t597 = t15 * t583;
    let t598 = 1.0_f64 / t597;
    (t593, t594, t596, t597, t598)
}
