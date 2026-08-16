//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 592/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk592(t1210: f64, t396: f64, t404: f64, t956: f64, t962: f64, t265: f64, t3005: f64, t3031: f64, t187: f64, t426: f64, t1236: f64, t1239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3573 = t1210 * t1210;
    let t3574 = 1.0_f64 / t3573;
    let t3575 = t396 * t3574;
    let t3576 = t404 * t404;
    let t3577 = 1.0_f64 / t3576;
    let t3582 = t956 * t962;
    let t3585 = t265 * t3005;
    let t3592 = t265 * t3031;
    let t3600 = t187 * t956;
    let t3621 = t426 * t426;
    let t3622 = 1.0_f64 / t3621;
    let t3638 = t1236 * t1239;
    (t3573, t3574, t3575, t3576, t3577, t3582, t3585, t3592, t3600, t3621, t3622, t3638)
}
