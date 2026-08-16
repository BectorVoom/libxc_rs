//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1053/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1053(t3107: f64, t27173: f64, t3183: f64, t3101: f64, t1135: f64, t8414: f64, t469: f64, t8995: f64, t454: f64, t509: f64, t25560: f64, t4456: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27203 = t3107 * t3107;
    let t27209 = t3183 * t27173;
    let t27215 = t3101 * t27173;
    let t27221 = t1135 * t8414;
    let t27276 = 1.0_f64 / t8995 / t469;
    let t27277 = t454 * t27276;
    let t27351 = t509 * t1135;
    let t27382 = t4456 * t25560;
    (t27203, t27209, t27215, t27221, t27277, t27351, t27382)
}
