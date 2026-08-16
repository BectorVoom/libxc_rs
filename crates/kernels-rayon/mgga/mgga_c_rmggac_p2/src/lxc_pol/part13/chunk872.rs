//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 872/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk872(t7255: f64, t8427: f64, t1965: f64, t9085: f64, t1969: f64, t1973: f64, t7259: f64, t8577: f64, t2305: f64, t35658: f64, t8497: f64, t35654: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39390 = t7255 * t8427;
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39394 = t39393 * t1973;
    let t39396 = t8577 * t7259;
    let t39401 = t35658 * t2305;
    let t39403 = t7255 * t8497;
    let t39405 = t35654 * t2305;
    (t39390, t39394, t39396, t39401, t39403, t39405)
}
