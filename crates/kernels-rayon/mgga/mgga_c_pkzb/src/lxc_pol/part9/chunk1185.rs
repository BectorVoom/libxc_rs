//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1185/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1185(t2865: f64, t5842: f64, t730: f64, t2866: f64, t5754: f64, t2848: f64, t5490: f64, t7227: f64, t1083: f64, t5802: f64, t17660: f64, t683: f64) -> (f64, f64, f64, f64) {
    let t20652 = 0.11696447245269292414e1_f64 * t730 * t2865 * t5842;
    let t20654 = 0.35089341735807877242e1_f64 * t5754 * t2866;
    let t20658 = 0.30762056574649219973e4_f64 * t730 * t5490 * t2848 * t7227;
    let t20659 = t5802 * t1083;
    let t20662 = 0.1551780387578202009e4_f64 * t20659 * t17660 * t683;
    (t20652, t20654, t20658, t20662)
}
