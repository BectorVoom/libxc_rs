//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1003/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1003(t1992: f64, t33249: f64, t80650: f64, t122166: f64, t6888: f64, t6891: f64, t115332: f64, t7691: f64, t6897: f64, t8621: f64, t90544: f64, t22633: f64, t22635: f64, t31558: f64, t97721: f64) -> (f64, f64, f64, f64, f64) {
    let t122370 = t1992 * t80650 * t33249;
    let t122377 = t6888 * t122166 * t6891;
    let t122384 = t6888 * t115332 * t7691;
    let t122390 = t6897 * t90544 * t8621;
    let t122394 = t22633 * t22635 * t31558 * t97721;
    (t122370, t122377, t122384, t122390, t122394)
}
