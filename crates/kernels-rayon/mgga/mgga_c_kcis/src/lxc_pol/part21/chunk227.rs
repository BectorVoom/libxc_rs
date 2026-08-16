//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 227/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk227(t920: f64, t943: f64, t924: f64, t935: f64, t940: f64, t947: f64) -> (f64, f64, f64) {
    let t964 = 0.301925e0_f64 * t920;
    let t967 = 0.82785e-1_f64 * t943;
    let t969 = 0.258925e1_f64 * t935 - t964 - 0.301925e0_f64 * t924 + 0.16504875e0_f64 * t940 - t967 - 0.82785e-1_f64 * t947;
    (t964, t967, t969)
}
