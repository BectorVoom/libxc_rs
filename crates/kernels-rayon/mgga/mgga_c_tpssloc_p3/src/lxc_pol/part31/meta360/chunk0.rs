//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1278/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1278(t12189: f64, t1804: f64, t5194: f64, t782: f64, t5198: f64, t3732: f64, t67: f64, t792: f64, t1799: f64, t212: f64, t1307: f64, t686: f64) -> (f64, f64, f64, f64, f64) {
    let t16078 = t12189 * t1804;
    let t16081 = t782 * t5194;
    let t16083 = 0.23333333333333333332e-1_f64 * t16081 * t5198;
    let t16093 = t3732 * t67;
    let t16094 = t792 * t16093;
    let t16095 = t212 * t1799;
    let t16097 = t686 * t16095 * t1307;
    (t16078, t16083, t16094, t16095, t16097)
}
