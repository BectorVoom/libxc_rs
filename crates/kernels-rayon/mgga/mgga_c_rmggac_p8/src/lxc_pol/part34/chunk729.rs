//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 729/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk729(t1322: f64, t235: f64, t36632: f64, t20: f64, t1311: f64, t1325: f64, t3054: f64, t641: f64, t70383: f64, t13809: f64, t7345: f64, t13815: f64, t2169: f64, t7553: f64) -> (f64, f64, f64, f64) {
    let t70585 = t235 * t36632 * t1322;
    let t70604 = t20 * t20;
    let t70610 = t1311 * t70604 * t3054 * t1322 * t1325 * t70383 * t641;
    let t70612 = t7345 * t13809;
    let t70618 = t7553 * t13815 * t2169;
    (t70585, t70610, t70612, t70618)
}
