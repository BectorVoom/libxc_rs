//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 926/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk926(t1469: f64, t34976: f64, t39851: f64, t571: f64, t39857: f64, t8417: f64, t8450: f64, t10102: f64, t34847: f64, t1528: f64, t3351: f64, t511: f64, t558: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t45436 = t39851 * t34976 * t571 * t1469;
    let t45439 = t8450 * t39857 * t8417;
    let t45441 = t34847 * t10102;
    let t45446 = t3351 * t7231 * t511 * t1528 * t558;
    (t45436, t45439, t45441, t45446)
}
