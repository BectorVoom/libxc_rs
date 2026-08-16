//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 127/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk127(t304: f64, t339: f64, t348: f64, t365: f64, t368: f64, t86: f64, t355: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t369 = t304 * t339;
    let t373 = 0.619125e-2_f64 * t365 * t348 - 0.39796666666666666666e-1_f64 * t86 * t368 * t369;
    let t374 = t373 * t355;
    let t375 = t374 * sigma0;
    (t369, t373, t374, t375)
}
