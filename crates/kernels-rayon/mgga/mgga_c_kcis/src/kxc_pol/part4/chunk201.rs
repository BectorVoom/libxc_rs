//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 201/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk201(t559: f64, t609: f64, t626: f64, t574: f64, t586: f64) -> (f64, f64, f64) {
    let t629 = t609 * t626 + 0.17411041666666666666e-2_f64 * t559;
    let t632 = 1.0_f64 + 0.9375e-1_f64 * t574 - 0.101171875e-1_f64 * t586;
    let t633 = 1.0_f64 / t632;
    (t629, t632, t633)
}
