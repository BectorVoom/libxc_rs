//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1369/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1369(t27173: f64, t3101: f64, t1135: f64, t8414: f64, t22035: f64, t894: f64, t2586: f64, t8512: f64, t1133: f64, t22041: f64, t3146: f64, t3151: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27215 = t3101 * t27173;
    let t27221 = t1135 * t8414;
    let t27223 = t894 * t27221 * t22035;
    let t27226 = t2586 * t8512;
    let t27227 = t1133 * t27226;
    let t27233 = t894 * t3146 * t22041;
    let t27237 = t894 * t3151 * t22041;
    (t27215, t27223, t27226, t27227, t27233, t27237)
}
