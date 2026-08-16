//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 132/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk132(t509: f64, t538: f64, t368: f64, t545: f64, t562: f64, t86: f64, t552: f64) -> (f64, f64, f64) {
    let t565 = t509 * t538;
    let t569 = 0.619125e-2_f64 * t562 * t545 - 0.39796666666666666666e-1_f64 * t86 * t368 * t565;
    let t570 = t569 * t552;
    (t565, t569, t570)
}
