//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 988/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk988(t32968: f64, t376: f64, t89: f64, t32972: f64, t1984: f64, t32869: f64, t32906: f64, t72: f64, t32988: f64, t375: f64, t23649: f64, t32926: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t139413 = t89 * t376 * t32968;
    let t139416 = t89 * t376 * t32972;
    let t139418 = t1984 * t32869;
    let t139431 = t72 * t32906;
    let t139453 = t89 * t375 * t32988;
    let t139485 = t23649 * t32926;
    (t139413, t139416, t139418, t139431, t139453, t139485)
}
