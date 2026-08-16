//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1414/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1414(t35482: f64, t36361: f64, t36362: f64, t36363: f64, t36364: f64, t36365: f64, t36366: f64, t36368: f64, t36369: f64, t36370: f64, t36371: f64, t36372: f64, t36373: f64) -> f64 {
    let t38545 = -t36361 - t36362 - t36363 + t36364 + t36365 - t36366 + 0.42242201367691890747e-5_f64 * t35482 - t36368 - t36369 + t36370 + t36371 - t36372 + t36373;
    t38545
}
