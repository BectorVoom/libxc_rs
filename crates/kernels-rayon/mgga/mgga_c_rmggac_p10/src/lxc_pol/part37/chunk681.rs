//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 681/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk681(t1322: f64, t507: f64, t7190: f64, t2046: f64, t641: f64, t7296: f64, t14327: f64, t333: f64, t3928: f64, t2048: f64, t338: f64, t352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68729 = t507 * t7190 * t1322;
    let t68735 = t2046 * t7296 * t641;
    let t68737 = t14327 * t333;
    let t68738 = t3928 * t68737;
    let t68739 = 0.23948483403727617128e0_f64 * t68738;
    let t68740 = t338 * t2048;
    let t68741 = t68740 * t352;
    (t68729, t68735, t68737, t68739, t68740, t68741)
}
