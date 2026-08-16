//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1363/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1363(t1122: f64, t1900: f64, t3119: f64, t11975: f64, t3116: f64, t2586: f64, t8956: f64, t1133: f64, t26255: f64, t8950: f64, t22035: f64, t894: f64) -> (f64, f64, f64, f64) {
    let t27122 = t1900 * t1122 * t3119;
    let t27124 = t3116 * t11975 * t27122;
    let t27126 = t2586 * t8956;
    let t27127 = t1133 * t27126;
    let t27129 = t8950 * t26255;
    let t27131 = t894 * t27129 * t22035;
    (t27124, t27126, t27127, t27131)
}
