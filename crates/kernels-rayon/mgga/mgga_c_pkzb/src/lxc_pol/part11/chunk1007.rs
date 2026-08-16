//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1007/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1007(t11100: f64, t790: f64, t11054: f64, t11064: f64, t11067: f64, t1134: f64, t1144: f64, t307: f64, t311: f64, t3670: f64, t3676: f64, t3695: f64) -> (f64, f64) {
    let t11101 = t790 * t11100;
    let t11104 = 0.65854491829355115987e0_f64 * t11054 * t311 - 0.19756347548806534796e1_f64 * t3670 * t1144 + 0.39512695097613069591e1_f64 * t1134 * t3676 - 0.19756347548806534796e1_f64 * t1134 * t3695 - 0.39512695097613069591e1_f64 * t307 * t11064 + 0.39512695097613069591e1_f64 * t307 * t11067 - 0.65854491829355115987e0_f64 * t307 * t11101;
    (t11101, t11104)
}
