//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 981/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk981(t2031: f64, t7768: f64, t7700: f64, t2039: f64, t7681: f64, t2105: f64, t2099: f64, t2947: f64, t2945: f64, t2003: f64, t2739: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7769 = t2031 * t7768;
    let t7770 = t7700 * t7769;
    let t7775 = t7681 * t2039;
    let t7776 = t2105 * t7775;
    let t7784 = t2099 * t2947;
    let t7786 = 0.17149607247227894789e-2_f64 * t2945 * t7784;
    let t7787 = t2003 * t2739;
    let t7788 = t7787 * t655;
    (t7769, t7770, t7775, t7776, t7784, t7786, t7787, t7788)
}
