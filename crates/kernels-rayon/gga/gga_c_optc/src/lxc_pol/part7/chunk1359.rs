//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1359/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1359(t3104: f64, t8905: f64, t1111: f64, t3088: f64, t530: f64, t24: f64, t8533: f64, t310: f64, t3648: f64, t449: f64, t448: f64, t123: f64, t3108: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27048 = t3104 * t8905;
    let t27053 = t1111 * t530 * t3088;
    let t27056 = t1111 * t24 * t8533;
    let t27059 = t310 * t3648 * t449;
    let t27061 = 0.18781521737197933637e-2_f64 * t448 * t27059;
    let t27063 = t3108 * t123 * t8905;
    (t27048, t27053, t27056, t27059, t27061, t27063)
}
