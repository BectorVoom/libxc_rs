//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 685/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk685(t1322: f64, t3924: f64, t507: f64, t14115: f64, t68420: f64, t1179: f64, t3116: f64, t3118: f64, t68527: f64, t14011: f64, t7477: f64, t68439: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68815 = t507 * t3924 * t1322;
    let t68844 = t68420 * t14115;
    let t68854 = t3116 * t1179 * t3118;
    let t68855 = t68527 * t68854;
    let t68856 = t14011 * t7477;
    let t68871 = t68439 * t14115;
    (t68815, t68844, t68854, t68855, t68856, t68871)
}
