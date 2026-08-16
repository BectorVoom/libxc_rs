//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1068/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1068(t19094: f64, t19097: f64, t291: f64, t33267: f64, t128: f64, t188: f64, t3707: f64) -> (f64, f64) {
    let t33270 = t19094 * t33267 * t291 * t19097;
    let t33273 = t3707 * t188 * t128;
    (t33270, t33273)
}
