//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 14/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk14(rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t39 = rho0 * rho0;
    let t40 = pow_1_3(rho0);
    let t41 = t40 * t40;
    let t43 = 1.0_f64 / t41 / t39;
    let t44 = sigma0 * t43;
    (t39, t40, t41, t43, t44)
}
