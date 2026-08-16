//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1330/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1330(t12435: f64, t12664: f64, t37331: f64, t37332: f64, t37333: f64, t37334: f64, t37335: f64, t37336: f64, t37337: f64, t38525: f64, t38526: f64, t38527: f64, t38528: f64, t38530: f64, t38859: f64, t38860: f64, t7: f64) -> f64 {
    let tv4rho2sigma29 = t37331 - t37332 + t37333 - t37334 + t37335 - t37336 - t37337 + t38525 - t38526 - t38527 + t38528 + t12435 + t38530 + t12664 + t7 * (t38859 + t38860);
    tv4rho2sigma29
}
