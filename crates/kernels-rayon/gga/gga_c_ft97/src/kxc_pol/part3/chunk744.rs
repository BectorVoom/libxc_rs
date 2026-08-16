//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 744/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk744(t15639: f64, t3020: f64, t401: f64, t4449: f64, t383: f64, t4441: f64, t35: f64, t7906: f64, t4467: f64, t4466: f64, t1594: f64, t428: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15640 = t3020 * t15639;
    let t15643 = t4449 * t401;
    let t15647 = t4441 * t383;
    let t15648 = t15647 * t35;
    let t15649 = t7906 * t15648;
    let t15652 = t4467 * t401;
    let t15656 = t4466 * t383;
    let t15657 = t15656 * t35;
    let t15658 = t1594 * t15657;
    let t15664 = t4449 * t428;
    (t15640, t15643, t15647, t15648, t15649, t15652, t15656, t15657, t15658, t15664)
}
