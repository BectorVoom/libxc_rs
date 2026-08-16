//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2032/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2032(t16081: f64, t5198: f64, t213: f64, t5187: f64, t1307: f64, t221: f64, t3719: f64, t5196: f64, t3732: f64, t67: f64, t792: f64) -> (f64, f64, f64, f64, f64) {
    let t16083 = 0.23333333333333333332e-1_f64 * t16081 * t5198;
    let t16084 = t213 * t5187;
    let t16086 = t221 * t16084 * t1307;
    let t16090 = t221 * t5196 * t3719;
    let t16093 = t3732 * t67;
    let t16094 = t792 * t16093;
    (t16083, t16086, t16090, t16093, t16094)
}
