//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 742/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk742(t8189: f64, t8192: f64, t8194: f64, t8197: f64, t8199: f64, t8201: f64, t8203: f64, t8205: f64) -> f64 {
    let t8251 = 0.9375e-1_f64 * t8189 - 0.9375e-1_f64 * t8192 - 0.25e0_f64 * t8194 + 0.625e-1_f64 * t8197 - 0.20234375e-1_f64 * t8199 + 0.20234375e-1_f64 * t8201 + 0.10791666666666666667e0_f64 * t8203 - 0.26979166666666666667e-1_f64 * t8205;
    t8251
}
