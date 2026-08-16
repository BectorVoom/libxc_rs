//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 764/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk764(t35786: f64, t638: f64, t7292: f64, t7385: f64, t2067: f64, t25640: f64, t2078: f64, t3851: f64, t7834: f64, t797: f64, t128: f64, t305: f64, t3899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35787 = 0.16432021104515675446e-2_f64 * t35786;
    let t35798 = t638 * t7292 * t7385;
    let t35799 = 0.12195059916630011326e-2_f64 * t35798;
    let t35810 = t25640 * t2067;
    let t35815 = t3851 * t2078;
    let t35824 = t797 * t7834;
    let t35861 = t305 * t128 * t3899;
    (t35787, t35799, t35810, t35815, t35824, t35861)
}
