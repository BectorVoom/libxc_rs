//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1316/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1316(t3229: f64, t3860: f64, t31052: f64, t31055: f64, t31057: f64, t31061: f64, t31092: f64, t31094: f64, t31096: f64, t31104: f64, t31106: f64, t31109: f64, t31111: f64, t31113: f64, t31115: f64, t31117: f64, t31122: f64, t31124: f64, t31591: f64, t31593: f64, t31595: f64, t31599: f64) -> (f64, f64) {
    let t31948 = t3860 * t3229;
    let t31950 = t31052 - t31055 - t31057 - t31061 - t31092 - t31094 + t31096 - t31104 - t31106 + t31109 - t31111 + t31113 + t31115 - t31117 - t31122 - t31124 + t31591 - t31593 - t31595 + t31599;
    (t31948, t31950)
}
