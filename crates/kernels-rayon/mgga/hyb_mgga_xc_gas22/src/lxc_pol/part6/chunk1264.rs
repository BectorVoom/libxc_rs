//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1264/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1264(t10119: f64, t1819: f64, t555: f64, t10123: f64, t8185: f64, t10137: f64, t10131: f64, t1782: f64, t1787: f64, t1796: f64, t1804: f64, t1807: f64, t1808: f64, t19698: f64, t19700: f64, t20006: f64, t23030: f64, t27035: f64, t27038: f64, t27066: f64, t27071: f64, t27085: f64, t3804: f64, t545: f64, t558: f64, t6164: f64, t6190: f64, t6195: f64, t7835: f64, t7842: f64, t9909: f64) -> f64 {
    let t27088 = t555 * t1819 * t10119;
    let t27091 = t555 * t8185 * t10123;
    let t27094 = t555 * t1819 * t10137;
    let t27096 = t27035 / 48.0_f64 + t7842 * t7835 * t27038 / 2.0_f64 - t23030 / 16.0_f64 + t19698 / 96.0_f64 + t19700 / 48.0_f64 + t20006 / 96.0_f64 - t555 * t558 * t6190 * t3804 / 64.0_f64 - t555 * t558 * t6195 * t3804 / 32.0_f64 - t555 * t558 * t1782 * t9909 / 32.0_f64 - t555 * t558 * t6164 * t3804 / 64.0_f64 - t555 * t558 * t1787 * t9909 / 32.0_f64 - t555 * t558 * t27066 * t545 / 32.0_f64 - t555 * t558 * t27071 * t545 / 32.0_f64 - t555 * t558 * t10131 * t1796 / 64.0_f64 - t1804 * t1807 * t10131 * t1808 / 48.0_f64 - t27085 / 48.0_f64 - t27088 / 48.0_f64 + 7.0_f64 / 48.0_f64 * t27091 - t27094 / 96.0_f64;
    t27096
}
