//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2297/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2297(t20148: f64, t580: f64, t20186: f64, t576: f64, t1395: f64, t6483: f64, t1404: f64, t6470: f64, t1858: f64, t5363: f64, t22430: f64, t111: f64, t20292: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t66967 = t20148 * t580;
    let t66976 = t576 * t20186;
    let t66987 = t1395 * t6483;
    let t66989 = t6470 * t1404;
    let t66991 = t5363 * t1858;
    let t67000 = t22430 * t580;
    let t67001 = t20292 * t111;
    (t66967, t66976, t66987, t66989, t66991, t67000, t67001)
}
