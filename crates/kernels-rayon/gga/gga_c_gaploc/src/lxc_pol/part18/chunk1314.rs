//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1314/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1314(t10938: f64, t2021: f64, t23310: f64, t25177: f64, t959: f64, t10847: f64, t22693: f64, t7572: f64, t24554: f64, t1: f64, t33137: f64, t20671: f64, t22538: f64, t24549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33565 = t2021 * t10938;
    let t33567 = 0.79445533226334281486e-1_f64 * t33565 * t23310;
    let t33568 = t25177 * t959;
    let t33569 = 0.29792074959875355558e-1_f64 * t33568;
    let t33572 = 0.18404604457881959845e2_f64 * t7572 * t22693 * t10847;
    let t33573 = t24554 * t959;
    let t33574 = 0.14896037479937677779e-1_f64 * t33573;
    let t33575 = t33137 * t1;
    let t33576 = t2021 * t33575;
    let t33580 = t22538 * t20671 * t24549;
    (t33567, t33569, t33572, t33574, t33576, t33580)
}
