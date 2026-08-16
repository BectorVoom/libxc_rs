//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1184/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1184(t1494: f64, t7257: f64, t167: f64, t1938: f64, t1380: f64, t6284: f64, t6281: f64, t1650: f64, t5732: f64, t22636: f64, t12234: f64, t7091: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t59071 = t1494 * t7257;
    let t59380 = t167 * t1938;
    let t59401 = t6284 * t1380;
    let t59414 = t6281 * t1380;
    let t59578 = t1650 * t5732;
    let t59975 = t22636 * sigma2;
    let t60029 = t7091 * t12234;
    (t59071, t59380, t59401, t59414, t59578, t59975, t60029)
}
