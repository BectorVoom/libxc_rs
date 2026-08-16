//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 147/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk147(t453: f64, t456: f64, t459: f64, t463: f64) -> (f64, f64, f64) {
    let t597 = 0.705945e1_f64 * t456 + 0.1549425e1_f64 * t453 + 0.420775e0_f64 * t459 + 0.1562925e0_f64 * t463;
    let t600 = 1.0_f64 + 0.32164683177870697974e2_f64 / t597;
    let t601 = f64::ln(t600);
    (t597, t600, t601)
}
