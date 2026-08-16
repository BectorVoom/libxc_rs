//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1060/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1060(t1804: f64, t3815: f64, t6214: f64, t125: f64, t3916: f64, t545: f64, t10102: f64, t10105: f64, t10107: f64, t10111: f64, t10115: f64, t10119: f64, t10123: f64, t1807: f64, t2987: f64, t555: f64, t558: f64, t5886: f64, t5889: f64, t7903: f64) -> (f64, f64, f64) {
    let t10129 = t1804 * t6214 * t3815;
    let t10131 = t3916 * t125;
    let t10132 = t10131 * t545;
    let t10136 = t5886 / 96.0_f64 + t5889 / 96.0_f64 - t10102 / 96.0_f64 - t10105 / 192.0_f64 - t1804 * t1807 * t10107 / 48.0_f64 - t1804 * t1807 * t10111 / 48.0_f64 - t555 * t558 * t10115 / 32.0_f64 - t555 * t558 * t10119 / 32.0_f64 - t555 * t2987 * t10123 / 16.0_f64 + t7903 / 144.0_f64 - t10129 / 144.0_f64 - t555 * t558 * t10132 / 64.0_f64;
    (t10131, t10132, t10136)
}
