//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1022/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1022(t16081: f64, t5198: f64, t3732: f64, t67: f64, t792: f64, t1799: f64, t212: f64, t1307: f64, t686: f64, t12214: f64, t131: f64, t205: f64) -> (f64, f64, f64, f64) {
    let t16083 = 0.23333333333333333332e-1_f64 * t16081 * t5198;
    let t16093 = t3732 * t67;
    let t16094 = t792 * t16093;
    let t16095 = t212 * t1799;
    let t16097 = t686 * t16095 * t1307;
    let t16099 = 0.49999999999999999998e-2_f64 * t16094 * t16097;
    let t16100 = t12214 * t131;
    let t16101 = t205 * t16100;
    (t16083, t16095, t16099, t16101)
}
