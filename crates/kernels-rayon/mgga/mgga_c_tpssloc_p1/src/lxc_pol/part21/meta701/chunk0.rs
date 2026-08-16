//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2530/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2530(t10186: f64, t13785: f64, t13839: f64, t2986: f64, t42837: f64, t10236: f64, t12652: f64, t12648: f64, t13783: f64, t1597: f64, t10237: f64, t340: f64, t4548: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48244 = t10186 * t13785;
    let t48250 = t2986 * t42837 * t13839;
    let t48256 = t10236 * t12652;
    let t48269 = t10236 * t12648;
    let t48279 = t13783 * t1597;
    let t48281 = t2986 * t48279 * t10237;
    let t48292 = t973 * t698 * t340 * t4548;
    (t48244, t48250, t48256, t48269, t48279, t48281, t48292)
}
