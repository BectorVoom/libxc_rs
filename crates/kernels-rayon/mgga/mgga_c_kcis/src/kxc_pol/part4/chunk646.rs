//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 646/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk646(t1220: f64, t3569: f64, t1210: f64, t396: f64, t404: f64, t3551: f64, t956: f64, t962: f64, t265: f64, t3005: f64, t3006: f64, t971: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3570 = t3569 * t1220;
    let t3573 = t1210 * t1210;
    let t3574 = 1.0_f64 / t3573;
    let t3575 = t396 * t3574;
    let t3576 = t404 * t404;
    let t3577 = 1.0_f64 / t3576;
    let t3578 = t3551 * t3577;
    let t3582 = t956 * t962;
    let t3585 = t265 * t3005;
    let t3586 = t3006 * t971;
    (t3570, t3573, t3574, t3575, t3576, t3577, t3578, t3582, t3585, t3586)
}
