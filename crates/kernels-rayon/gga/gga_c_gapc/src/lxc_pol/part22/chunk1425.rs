//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1425/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1425(t12042: f64, t12151: f64, t36091: f64, t36092: f64, t36093: f64, t37313: f64, t37314: f64, t37317: f64, t37318: f64, t37319: f64, t37320: f64, t37322: f64, t37323: f64, t37324: f64, t37325: f64, t37326: f64, t37327: f64, t37328: f64, t37329: f64, t7: f64) -> f64 {
    let t37330 = 2.0_f64 * t12042;
    let tv4rho2sigma21 = -t36091 - t36092 + t36093 + t7 * (t37313 + t37314) + t37317 - t37318 + t37319 - t37320 + 2.0_f64 * t12151 + t37322 - t37323 + t37324 + t37325 - t37326 - t37327 + t37328 + t37329 - t37330;
    tv4rho2sigma21
}
