//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 651/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk651(t1509: f64, t5021: f64, t193: f64, t670: f64, t22: f64, t137: f64, t647: f64) -> (f64, f64, f64, f64) {
    let t5022 = t5021 * t1509;
    let t5054 = t670 * t193;
    let t5056 = 1.0_f64 / t22 / t5054;
    let t5059 = t647 * t137;
    (t5022, t5054, t5056, t5059)
}
