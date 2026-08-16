//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1105/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1105(t5864: f64, t7822: f64, t5869: f64, t5895: f64, t5899: f64, t1844: f64, t1983: f64, t7585: f64, t7586: f64, t1750: f64, t30948: f64, t1165: f64, t30806: f64, t5824: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39080 = t7822 * t5864;
    let t39082 = t7822 * t5869;
    let t39086 = t7822 * t5895;
    let t39088 = t7822 * t5899;
    let t39092 = t7585 * t7586 * t1983 * t1844;
    let t39094 = t30948 * t1750;
    let t39098 = t30806 * t1165 * t604 * t5824;
    (t39080, t39082, t39086, t39088, t39092, t39094, t39098)
}
