//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1044/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1044(t16144: f64, t5564: f64, t659: f64, t16050: f64, t16048: f64, t127: f64, t368: f64, t3751: f64, t1477: f64, t3754: f64, t1482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16145 = 0.21908444444444444444e0_f64 * t16144;
    let t16146 = t659 * t5564;
    let t16156 = 0.39862222222222222222e0_f64 * t16050;
    let t16183 = 4.0_f64 / 27.0_f64 * t16048;
    let t16184 = 4.0_f64 / 9.0_f64 * t16050;
    let t16232 = 0.41203703703703703704e-2_f64 * t16048;
    let t16233 = 0.12361111111111111111e-1_f64 * t16050;
    let t16292 = 0.22076e0_f64 * t16144;
    let t16301 = 0.13418888888888888889e0_f64 * t16048;
    let t16353 = t127 * t368 * t3751;
    let t16354 = t1477 * t3754;
    let t16359 = t1482 * t3754;
    (t16145, t16146, t16156, t16183, t16184, t16232, t16233, t16292, t16301, t16353, t16354, t16359)
}
