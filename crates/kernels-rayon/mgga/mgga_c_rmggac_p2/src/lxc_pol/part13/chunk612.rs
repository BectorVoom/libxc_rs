//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 612/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk612(t8041: f64, t876: f64, t1356: f64, t699: f64, t839: f64, t1364: f64, t702: f64, t934: f64) -> (f64, f64, f64, f64, f64) {
    let t8042 = t8041 * t876;
    let t8043 = t1356 * t8042;
    let t8044 = 0.11974241701863808564e0_f64 * t8043;
    let t8045 = t699 * t839;
    let t8046 = t1364 * t8045;
    let t8047 = 0.23948483403727617128e0_f64 * t8046;
    let t8048 = t934 * t702;
    (t8042, t8044, t8045, t8047, t8048)
}
