//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2561/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2561(t10903: f64, t14507: f64, t14651: f64, t3069: f64, t10956: f64, t1611: f64, t10517: f64, t4630: f64, t10459: f64, t4644: f64, t4608: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50302 = t14507 * t10903;
    let t50324 = t14651 * t3069;
    let t50334 = t1611 * t10956;
    let t50337 = t10517 * t4630;
    let t50343 = t4644 * t10459;
    let t50361 = t973 * t698 * t4608;
    (t50302, t50324, t50334, t50337, t50343, t50361)
}
