//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 110/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk110(t214: f64, t217: f64, t220: f64, t226: f64) -> (f64, f64, f64) {
    let t261 = 0.51785e1_f64 * t217 + 0.905775e0_f64 * t214 + 0.1100325e0_f64 * t220 + 0.1241775e0_f64 * t226;
    let t264 = 1.0_f64 + 0.29608574643216675549e2_f64 / t261;
    let t265 = f64::ln(t264);
    (t261, t264, t265)
}
