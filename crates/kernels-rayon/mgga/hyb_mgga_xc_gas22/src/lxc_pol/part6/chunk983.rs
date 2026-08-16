//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 983/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk983(t2524: f64, t9104: f64, t1410: f64, t7065: f64, t2473: f64, t3514: f64, t238: f64, t3505: f64, t801: f64, t3509: f64, t1392: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9106 = 0.16081979498692535067e2_f64 * t9104 * t2524;
    let t9108 = 1.0_f64 * t7065 * t1410;
    let t9110 = 2.0_f64 * t2473 * t3514;
    let t9112 = t238 * t801 * t3505;
    let t9113 = 0.32862666666666666666e0_f64 * t9112;
    let t9115 = t238 * t801 * t3509;
    let t9116 = 0.32862666666666666666e0_f64 * t9115;
    let t9117 = t2466 * t1392;
    (t9106, t9108, t9110, t9112, t9113, t9115, t9116, t9117)
}
