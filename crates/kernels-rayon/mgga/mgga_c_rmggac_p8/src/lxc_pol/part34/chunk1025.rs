//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1025/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1025(t71583: f64, t75729: f64, t2211: f64, t739: f64, t8915: f64, t699: f64, t8712: f64, t903: f64, t15523: f64, t2186: f64, t15598: f64, t321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77803 = 0.96056421943322389208e-3_f64 * t71583;
    let t77804 = 0.16351352353374609375e-5_f64 * t75729;
    let t77806 = t739 * t2211 * t8915;
    let t77807 = 0.79828278012425390427e-1_f64 * t77806;
    let t77809 = t903 * t699 * t8712;
    let t77810 = 0.11974241701863808564e0_f64 * t77809;
    let t77811 = t2186 * t15523;
    let t77812 = 0.99317399751028291929e-5_f64 * t77811;
    let t77816 = t15598 * t321;
    (t77803, t77804, t77807, t77810, t77812, t77816)
}
