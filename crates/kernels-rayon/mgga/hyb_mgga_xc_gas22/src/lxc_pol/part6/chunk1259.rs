//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1259/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1259(t26727: f64, t2922: f64, t654: f64, t198: f64, t1143: f64, t9586: f64, t2837: f64, t524: f64, t7744: f64, t3746: f64, t3785: f64, t2875: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26728 = t2922 * t26727;
    let t26729 = t654 * tau1;
    let t26730 = t26729 * t198;
    let t26846 = t1143 * t9586;
    let t26850 = t524 * t2837 * t7744;
    let t26865 = t3785 * t3746;
    let t26883 = t2875 * t26727;
    (t26728, t26729, t26730, t26846, t26850, t26865, t26883)
}
