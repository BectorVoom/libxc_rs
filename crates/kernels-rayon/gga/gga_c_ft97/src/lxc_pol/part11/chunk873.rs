//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 873/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk873(t1691: f64, t44: f64, t5588: f64, t8155: f64, t7853: f64, t32211: f64, t45: f64, t1690: f64, t1692: f64, t1630: f64, t383: f64, t55: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37894 = t1691 * t1691;
    let t37897 = 1.0_f64 / t44 / t8155 / t5588;
    let t37899 = t37894 * t37897 * t7853;
    let t37903 = 1.0_f64 / t45 / t32211;
    let t37905 = t1690 * t1692 * t37903;
    let t37908 = t1630 * t383;
    let t37930 = t8155 * t55;
    (t37894, t37897, t37899, t37903, t37905, t37908, t37930)
}
