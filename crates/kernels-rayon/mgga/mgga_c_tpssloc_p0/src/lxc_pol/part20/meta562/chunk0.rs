//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2120/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2120(t42340: f64, t42341: f64, t3034: f64, t368: f64, t3128: f64, t10882: f64, t42333: f64, t1015: f64, t1041: f64, t10914: f64, t13969: f64, t10918: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42342 = t42340 * t42341;
    let t42343 = t3034 * t3034;
    let t42344 = 1.0_f64 / t42343;
    let t42345 = t368 * t42344;
    let t42347 = t42342 * t3128 * t42345;
    let t42354 = t42333 * t10882;
    let t42358 = t42342 * t1015 * t42345;
    let t42369 = t1041 * t13969 * t10914;
    let t42372 = t1041 * t13969 * t10918;
    (t42342, t42344, t42345, t42347, t42354, t42358, t42369, t42372)
}
