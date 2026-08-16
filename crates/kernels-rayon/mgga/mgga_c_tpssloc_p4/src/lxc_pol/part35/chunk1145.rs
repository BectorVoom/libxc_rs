//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1145/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1145(t2148: f64, t3427: f64, t2121: f64, t225: f64, t24594: f64, t23598: f64, t50: f64, t131: f64, t467: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24771 = t3427 * t2148;
    let t24773 = 0.18277045187202515961e-2_f64 * t2121 * t24771;
    let t24776 = t24594 * t225;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    (t24771, t24773, t24776, t24810, t24811, t24812)
}
