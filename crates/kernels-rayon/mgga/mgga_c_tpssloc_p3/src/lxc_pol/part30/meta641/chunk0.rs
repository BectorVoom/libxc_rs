//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2051/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2051(t14137: f64, t6765: f64, t7583: f64, t83138: f64, t25644: f64, t82926: f64, t23512: f64, t25486: f64, t23519: f64, t25492: f64, t1597: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88339 = 5.0_f64 / 5184.0_f64 * t6765 * t14137;
    let t88341 = 0.20186378047070195428e-3_f64 * t83138 * t7583;
    let t88348 = t82926 * t25644;
    let t88351 = t23512 * t25486;
    let t88354 = t23519 * t25492;
    let t88360 = t607 * t1597;
    (t88339, t88341, t88348, t88351, t88354, t88360)
}
