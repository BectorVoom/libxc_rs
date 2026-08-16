//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1031/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1031(t1628: f64, t7533: f64, t1636: f64, t7537: f64, t2128: f64, t6256: f64, t7566: f64, t22349: f64, t22352: f64, t22355: f64, t22359: f64, t22362: f64, t22365: f64, t22367: f64, t22369: f64, t22371: f64, t22374: f64, t22377: f64) -> (f64, f64, f64, f64, f64) {
    let t23255 = t7533 * t1628;
    let t23265 = t7537 * t1636;
    let t23268 = t2128 * t6256;
    let t23272 = t7566 * t1636;
    let t23297 = -0.20234375e-1_f64 * t22349 + 0.375e0_f64 * t22352 + 0.89930555555555555553e-2_f64 * t22355 - 0.9375e-1_f64 * t22359 + 0.1875e0_f64 * t22362 + 0.13489583333333333333e-1_f64 * t22365 - 0.14388888888888888889e0_f64 * t22367 - 0.1875e0_f64 * t22369 - 0.14388888888888888889e0_f64 * t22371 + 0.125e0_f64 * t22374 + 0.27777777777777777777e-1_f64 * t22377;
    (t23255, t23265, t23268, t23272, t23297)
}
