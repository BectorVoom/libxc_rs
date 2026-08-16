//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2198/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2198(t11148: f64, t1227: f64, t248: f64, t45268: f64, t11728: f64, t11729: f64, t3570: f64, t1229: f64, t204: f64, t1090: f64, t3609: f64, t44927: f64) -> (f64, f64, f64, f64, f64) {
    let t45271 = t1227 * t248 * t45268 * t11148;
    let t45283 = t11728 * t248 * t3570 * t11729;
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    let t45320 = t44927 * t3609;
    (t45271, t45283, t45293, t45296, t45320)
}
