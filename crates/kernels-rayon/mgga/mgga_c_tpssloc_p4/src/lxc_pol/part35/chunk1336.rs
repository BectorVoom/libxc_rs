//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1336/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1336(t24574: f64, t29790: f64, t29763: f64, t8067: f64, t94490: f64, t27604: f64, t4993: f64, t19095: f64, t24733: f64, t1207: f64, t19024: f64, t7337: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103950 = t24574 * t29790;
    let t103954 = t24574 * t29763;
    let t103959 = t94490 * t8067;
    let t104007 = t27604 * t4993;
    let t104009 = t24733 * t19095;
    let t104012 = t1207 * t7337 * t19024;
    (t103950, t103954, t103959, t104007, t104009, t104012)
}
