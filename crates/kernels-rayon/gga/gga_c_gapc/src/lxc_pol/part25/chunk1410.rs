//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1410/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1410(t35422: f64, t35451: f64, t36346: f64, t36347: f64, t36349: f64, t36350: f64, t36351: f64, t36352: f64, t36353: f64, t36354: f64, t36355: f64, t36357: f64, t36358: f64) -> f64 {
    let t38542 = t36346 + t36347 - 0.5431140175846100239e-5_f64 * t35422 + t36349 - t36350 - t36351 - t36352 - t36353 - t36354 + t36355 - 0.49106559089941822994e-4_f64 * t35451 + t36357 + t36358;
    t38542
}
