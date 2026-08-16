//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 728/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk728(t14226: f64, t70548: f64, t14020: f64, t68536: f64, t14019: f64, t14027: f64, t14267: f64, t2165: f64, t3056: f64, t2169: f64, t2046: f64, t2049: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t70549 = t70548 * t14226;
    let t70554 = t14020 * t68536;
    let t70556 = t14019 * t70554 * t14027;
    let t70573 = t3056 * t14267 * t2165;
    let t70577 = t3056 * t14267 * t2169;
    let t70582 = t2046 * t2049 * t2165;
    (t70549, t70554, t70556, t70573, t70577, t70582)
}
