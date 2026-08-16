//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 90/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk90(t260: f64, t105: f64, t107: f64, t269: f64) -> (f64, f64) {
    let t273 = t260 * t260;
    let t275 = 0.50765919958333333334e-3_f64 * t105 * t107 * t269 - 2.0_f64 * t273;
    let t276 = 1.0_f64 / t275;
    (t275, t276)
}
