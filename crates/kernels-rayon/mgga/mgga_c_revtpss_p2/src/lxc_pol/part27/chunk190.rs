//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 190/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk190(t607: f64, t70: f64, t39: f64, t41: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t608 = t607 * t70;
    let t611 = t39 * rho0;
    let t613 = 1.0_f64 / t41 / t611;
    let t614 = sigma0 * t613;
    (t608, t611, t613, t614)
}
