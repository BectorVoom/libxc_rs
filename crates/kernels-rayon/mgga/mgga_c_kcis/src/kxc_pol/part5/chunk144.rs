//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 144/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk144(t187: f64, t394: f64, t433: f64, t437: f64, rho1: f64, sigma2: f64) -> (f64, f64) {
    let t441 = t394 + t187 * (t433 * t437 - t394);
    let t445 = 1.0_f64 / rho1;
    let t446 = sigma2 * t445;
    (t441, t446)
}
