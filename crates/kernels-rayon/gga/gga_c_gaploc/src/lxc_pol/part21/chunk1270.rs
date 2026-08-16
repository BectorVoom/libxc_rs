//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1270/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1270(t33557: f64, t6066: f64, t7630: f64, t32356: f64, t739: f64, t1991: f64, t590: f64, t10938: f64, t2021: f64, t23310: f64, t25177: f64, t959: f64) -> (f64, f64, f64, f64) {
    let t33560 = 0.14300195980740170668e1_f64 * t7630 * t6066 * t33557;
    let t33561 = t739 * t32356;
    let t33564 = 0.2044956050875773316e1_f64 * t1991 * t33561 * t590;
    let t33565 = t2021 * t10938;
    let t33567 = 0.79445533226334281486e-1_f64 * t33565 * t23310;
    let t33568 = t25177 * t959;
    (t33560, t33564, t33567, t33568)
}
