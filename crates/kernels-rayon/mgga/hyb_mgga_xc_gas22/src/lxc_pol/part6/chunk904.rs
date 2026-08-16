//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 904/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk904(t1787: f64, t3: f64, t1184: f64, t555: f64, t6160: f64, t1782: f64, t1179: f64, t6164: f64, t125: f64, t3112: f64, t545: f64, t2987: f64, t558: f64, t5871: f64, t5874: f64, t5876: f64, t5880: f64, t5881: f64, t5883: f64, t5886: f64, t5889: f64, t5904: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7898 = t1787 * t3;
    let t7903 = t555 * t6160 * t1184;
    let t7905 = t1782 * t3;
    let t7909 = t6164 * t1179;
    let t7913 = t3112 * t125;
    let t7914 = t7913 * t545;
    let t7918 = -t5871 / 32.0_f64 - t5874 / 64.0_f64 + t5876 / 48.0_f64 - t5880 - t5881 / 32.0_f64 - t5883 / 32.0_f64 + t5886 / 48.0_f64 + t5889 / 48.0_f64 - t5904 / 64.0_f64 - t555 * t2987 * t7898 / 16.0_f64 + t7903 / 288.0_f64 - t555 * t2987 * t7905 / 16.0_f64 - t555 * t558 * t7909 / 64.0_f64 - t555 * t558 * t7914 / 32.0_f64;
    (t7898, t7903, t7905, t7909, t7913, t7914, t7918)
}
