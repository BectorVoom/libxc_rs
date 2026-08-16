//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 974/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk974(t10051: f64, t21531: f64, t1882: f64, t21693: f64, t21505: f64, t8392: f64, t21678: f64, t21524: f64, t21399: f64, t258: f64, t21757: f64, t21732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t81302 = t10051 * t21531;
    let t81334 = t1882 * t21693;
    let t81358 = t8392 * t21505;
    let t81365 = t1882 * t21678;
    let t81411 = t1882 * t21524;
    let t81413 = t258 * t21399;
    let t81448 = t8392 * t21757;
    let t81454 = t1882 * t21732;
    (t81302, t81334, t81358, t81365, t81411, t81413, t81448, t81454)
}
