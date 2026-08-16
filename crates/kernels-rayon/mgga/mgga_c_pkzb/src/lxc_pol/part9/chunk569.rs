//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 569/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk569(t2363: f64, t326: f64, t2023: f64, t401: f64, t46: f64, t919: f64) -> (f64, f64, f64, f64, f64) {
    let t2364 = t2363 * t326;
    let t2365 = t401 * t2023;
    let t2366 = t2365 * t46;
    let t2367 = t2364 * t2366;
    let t2368 = t919 * t919;
    (t2364, t2365, t2366, t2367, t2368)
}
