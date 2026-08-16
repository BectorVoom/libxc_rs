//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 886/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk886(t11: f64, t1690: f64, t7868: f64, t5544: f64, t8018: f64, t1685: f64, t1597: f64, t1663: f64, t78: f64, t8153: f64, t8157: f64, t32: f64, t8991: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38176 = t1690 * t11;
    let t38177 = t38176 * t7868;
    let t38180 = t5544 * t8018;
    let t38187 = t1685 * t1685;
    let t38192 = t1597 * t1663 * t78;
    let t38195 = t8153 * t8157;
    let t38200 = t8991 / t32;
    (t38177, t38180, t38187, t38192, t38195, t38200)
}
