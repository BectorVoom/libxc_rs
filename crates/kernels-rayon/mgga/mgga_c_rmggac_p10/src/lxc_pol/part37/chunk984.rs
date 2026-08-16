//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 984/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk984(t75729: f64, t2211: f64, t739: f64, t8915: f64, t699: f64, t8712: f64, t903: f64, t15523: f64, t2186: f64, t71594: f64, t14441: f64, t5928: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77804 = 0.16351352353374609375e-5_f64 * t75729;
    let t77806 = t739 * t2211 * t8915;
    let t77807 = 0.79828278012425390427e-1_f64 * t77806;
    let t77809 = t903 * t699 * t8712;
    let t77810 = 0.11974241701863808564e0_f64 * t77809;
    let t77811 = t2186 * t15523;
    let t77812 = 0.99317399751028291929e-5_f64 * t77811;
    let t77820 = 0.15243824895787514157e-3_f64 * t71594;
    let t77823 = 0.39914139006212695214e-1_f64 * t5928 * t14441;
    (t77804, t77807, t77810, t77812, t77820, t77823)
}
