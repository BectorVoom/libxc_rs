//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2245/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2245(t10937: f64, t18041: f64, t1041: f64, t13969: f64, t17636: f64, t17642: f64, t17906: f64, t3117: f64, t17624: f64, t2960: f64, t5884: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62499 = t10937 * t18041;
    let t62510 = t1041 * t13969 * t17636;
    let t62515 = t1041 * t13969 * t17642;
    let t62534 = t3117 * t17906;
    let t62556 = t2960 * t17624;
    let t62559 = t973 * t698 * t5884;
    (t62499, t62510, t62515, t62534, t62556, t62559)
}
