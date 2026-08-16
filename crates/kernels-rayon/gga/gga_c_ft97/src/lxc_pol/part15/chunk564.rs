//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 564/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk564(t24: f64, t7241: f64, t486: f64, t100: f64, t1570: f64, t487: f64, t8189: f64, t8326: f64, t104: f64, t7943: f64, t89: f64, t1786: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8411 = t24 * t7241;
    let t8416 = t486 * t486;
    let t8417 = 1.0_f64 / t8416;
    let t8418 = t100 * t8417;
    let t8424 = t487 * t1570;
    let t8455 = 28.0_f64 / 81.0_f64 * t8189;
    let t8518 = t8326 * t100;
    let t8534 = 28.0_f64 / 81.0_f64 * t89 * t7943 * t104;
    let t8557 = t1786 * t488;
    (t8411, t8416, t8417, t8418, t8424, t8455, t8518, t8534, t8557)
}
