//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1153/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1153(t1143: f64, t3687: f64, t524: f64, t9573: f64, t1166: f64, t3660: f64, t536: f64, t9531: f64, t2867: f64, t525: f64, t17: f64, t7692: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14765 = t1143 * t3687;
    let t14770 = t524 * t9573;
    let t14775 = t1166 * t3660;
    let t14815 = t536 * t9531;
    let t14818 = t2867 * t525;
    let t15041 = t7692 * t17;
    (t14765, t14770, t14775, t14815, t14818, t15041)
}
