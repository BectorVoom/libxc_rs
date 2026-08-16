//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1195/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1195(t218: f64, t675: f64, t7391: f64, t7395: f64, t7399: f64, t1066: f64, t219: f64, t5537: f64, t1843: f64, t2739: f64, t655: f64, t7350: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20759 = t218 * t675 * t7391;
    let t20762 = t218 * t675 * t7395;
    let t20765 = t218 * t675 * t7399;
    let t20769 = t218 * t219 * t5537 * t1066;
    let t20773 = t218 * t219 * t1843 * t2739;
    let t20777 = t218 * t219 * t655 * t7350;
    (t20759, t20762, t20765, t20769, t20773, t20777)
}
