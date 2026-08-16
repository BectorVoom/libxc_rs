//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2443/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2443(t42891: f64, t973: f64, t979: f64, t2955: f64, t2967: f64, t2402: f64, t986: f64, t3010: f64, t698: f64, t10286: f64, t2960: f64, t3016: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42893 = t973 * t42891 * t979;
    let t42895 = t2955 * t2967;
    let t42903 = t973 * t2402 * t986;
    let t42906 = t973 * t698 * t3010;
    let t42911 = t2960 * t10286;
    let t42914 = t973 * t698 * t3016;
    (t42893, t42895, t42903, t42906, t42911, t42914)
}
