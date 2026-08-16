//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 438/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk438(t4461: f64, t465: f64, t479: f64, t198: f64, t2184: f64, t1193: f64, t1198: f64, t1190: f64, t1219: f64, t1212: f64, t209: f64, t1180: f64, t1189: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4555 = t465 * t4461;
    let t4556 = t4555 * t479;
    let t4558 = t2184 * t198;
    let t4559 = t1193 * t4558;
    let t4560 = t4559 * t1198;
    let t4562 = t1190 * t1219;
    let t4564 = t1212 * t209;
    let t4569 = t1180 * t1189;
    (t4555, t4556, t4559, t4560, t4562, t4564, t4569)
}
