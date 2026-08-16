//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 967/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk967(t10556: f64, t50: f64, t581: f64, t10502: f64, t1024: f64, t3396: f64, t1034: f64, t3441: f64, t164: f64, t179: f64, t2593: f64, t1020: f64, t8904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10557 = t50 * t10556;
    let t10558 = t581 * t10557;
    let t10561 = t50 * t10502;
    let t10562 = t581 * t10561;
    let t10566 = t581 * t1024 * t3396;
    let t10572 = t3441 * t1034;
    let t10573 = t10572 * t164;
    let t10574 = t179 * t10573;
    let t10577 = t2593 * t3441;
    let t10578 = t179 * t10577;
    let t10582 = t179 * t8904 * t1020;
    (t10557, t10558, t10561, t10562, t10566, t10572, t10573, t10574, t10577, t10578, t10582)
}
