//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 725/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk725(t570: f64, t794: f64, t1652: f64, t352: f64, t551: f64, t866: f64, t848: f64, t1587: f64, t838: f64, t874: f64, t839: f64, t558: f64, t876: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27111 = t570 * t794;
    let t27120 = t1652 * t352;
    let t27124 = t551 * t866;
    let t27136 = t570 * t848;
    let t27146 = t1587 * t352;
    let t27176 = t838 * t874;
    let t27177 = t570 * t839;
    let t27326 = t558 * t876;
    (t27111, t27120, t27124, t27136, t27146, t27176, t27177, t27326)
}
