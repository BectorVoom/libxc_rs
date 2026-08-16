//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 320/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk320(t529: f64, t547: f64, t524: f64, t1216: f64, t41: f64, t1287: f64, t382: f64, t525: f64, t526: f64, t79: f64, t534: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t530 = t529 < -0.66725e-1_f64;
    let t1555 = t547 * t547;
    let t1556 = 1.0_f64 / t1555;
    let t1557 = t524 * t1556;
    let t1558 = t1216 * t41;
    let t1566 = piecewise3(t530, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t1558 * t382 - 10.0_f64 / 27.0_f64 * t525 * t526 * t1287);
    let t1567 = t79 * t1566;
    let t1568 = t1567 * t534;
    (t1555, t1556, t1557, t1558, t1567, t1568)
}
