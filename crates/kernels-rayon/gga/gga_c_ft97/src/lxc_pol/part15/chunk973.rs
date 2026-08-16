//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 973/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk973(t21400: f64, t375: f64, t89: f64, t21431: f64, t41962: f64, t1882: f64, t21519: f64, t21736: f64, t21728: f64, t21740: f64, t21533: f64, t21661: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t81124 = t89 * t375 * t21400;
    let t81131 = t89 * t41962 * t21431;
    let t81162 = t1882 * t21519;
    let t81164 = t1882 * t21736;
    let t81170 = t1882 * t21728;
    let t81183 = t1882 * t21740;
    let t81207 = t1882 * t21533;
    let t81209 = t8392 * t21661;
    (t81124, t81131, t81162, t81164, t81170, t81183, t81207, t81209)
}
