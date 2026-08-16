//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 966/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk966(t7550: f64, t7574: f64, t301: f64, t761: f64, t758: f64, t1125: f64, t5939: f64, t757: f64, t2096: f64, t2908: f64, t2886: f64, t434: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7575 = t7550 + t7574;
    let t7577 = t301 * t7575 * t761;
    let t7578 = t758 * t7577;
    let t7581 = t5939 * t1125;
    let t7582 = t757 * t7581;
    let t7585 = 0.15244095330869239812e-2_f64 * t2096 * t2908;
    let t7586 = t434 * t2886;
    (t7575, t7577, t7578, t7581, t7582, t7585, t7586)
}
