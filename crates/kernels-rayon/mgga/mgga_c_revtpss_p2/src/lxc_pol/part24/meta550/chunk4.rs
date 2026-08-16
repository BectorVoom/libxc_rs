//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1630/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1630(t14586: f64, t1559: f64, t18677: f64, t4504: f64, t4514: f64, t51578: f64, t51635: f64, t6017: f64, t62909: f64, t62920: f64, t62922: f64, t62952: f64, t62983: f64, t62999: f64, t77159: f64, t77225: f64, t820: f64) -> f64 {
    let t87869 = 0.7805952431506226415e-2_f64 * t62909 + 0.39029762157531132075e-2_f64 * t62920 - 0.87805989105806821314e-1_f64 * t62922 + 0.15805078039045227836e2_f64 * t4504 * t77159 * t14586 - 0.44178176337912614788e-3_f64 * t51578 + 0.78059524315062264152e-1_f64 * t62952 - 0.26341796731742046395e1_f64 * t820 * t77225 * t1559 + 0.1561190486301245283e0_f64 * t62983 + 0.18505311230957427423e-1_f64 * t51635 - 0.39512695097613069592e1_f64 * t4514 * t18677 * t6017 - 0.69394917116090352835e-2_f64 * t62999;
    t87869
}
