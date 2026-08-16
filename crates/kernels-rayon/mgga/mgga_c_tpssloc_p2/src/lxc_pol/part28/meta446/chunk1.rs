//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1632/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1632(t109: f64, t22468: f64, t22471: f64, t22474: f64, t22476: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t23912 = 22.0_f64 / 9.0_f64 * t22468;
    let t23917 = piecewise3(t110, 0.0_f64, t23912 + 4.0_f64 / 3.0_f64 * t22471 + t22474 / 2.0_f64 - t22476 / 4.0_f64);
    (t23912, t23917)
}
