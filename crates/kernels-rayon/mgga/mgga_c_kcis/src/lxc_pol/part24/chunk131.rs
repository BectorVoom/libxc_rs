//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 131/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk131(t304: f64, t339: f64, t348: f64, t365: f64, t368: f64, t86: f64, t355: f64) -> (f64, f64, f64) {
    let t369 = t304 * t339;
    let t373 = 0.619125e-2_f64 * t365 * t348 - 0.39796666666666666666e-1_f64 * t86 * t368 * t369;
    let t374 = t373 * t355;
    (t369, t373, t374)
}
