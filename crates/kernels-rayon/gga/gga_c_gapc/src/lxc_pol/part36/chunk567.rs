//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 567/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk567(t3357: f64, t943: f64, t3056: f64, t325: f64, t122: f64, t761: f64) -> (f64, f64, f64, f64) {
    let t3358 = t3357 * t943;
    let t3360 = t325 * t3056;
    let t3361 = t3360 * t943;
    let t3363 = t761 * t122;
    (t3358, t3360, t3361, t3363)
}
