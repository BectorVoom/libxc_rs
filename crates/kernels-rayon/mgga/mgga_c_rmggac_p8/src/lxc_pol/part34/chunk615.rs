//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 615/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk615(t15494: f64, t2474: f64, t640: f64, t638: f64, t639: f64, t3219: f64, t8571: f64, t618: f64, t698: f64) -> (f64, f64, f64, f64, f64) {
    let t15495 = 0.15243824895787514157e-3_f64 * t15494;
    let t15496 = t640 * t2474;
    let t15498 = t638 * t639 * t15496;
    let t15499 = 0.15243824895787514157e-3_f64 * t15498;
    let t15500 = t8571 * t3219;
    let t15501 = 0.42564599893297839398e-5_f64 * t15500;
    let t15502 = t698 * t618;
    (t15495, t15496, t15499, t15501, t15502)
}
