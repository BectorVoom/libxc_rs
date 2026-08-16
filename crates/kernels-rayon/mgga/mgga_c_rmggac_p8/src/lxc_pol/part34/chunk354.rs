//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 354/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk354(t20: f64, t252: f64, t1318: f64, t40: f64, t41: f64, t21: f64, t1342: f64, t45: f64) -> (f64, f64, f64, f64, f64) {
    let t4738 = t252 * t20;
    let t4762 = t1318 * t40;
    let t4764 = 1.0_f64 / t41 / t4762;
    let t4765 = t21 * t4764;
    let t4789 = 1.0_f64 / t1342 / t45;
    (t4738, t4762, t4764, t4765, t4789)
}
