//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 788/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk788(t9263: f64, t9278: f64, t993: f64, t20671: f64, t31041: f64, t34818: f64, t34264: f64, t7030: f64, t10177: f64, t10523: f64, t544: f64, t899: f64, t913: f64) -> (f64, f64, f64, f64) {
    let t41686 = t9263 * t993 * t9278;
    let t41689 = t31041 * t20671 * t34818;
    let t41691 = t34264 * t7030;
    let t41696 = t544 * t10523 * t899 * t913 * t10177;
    (t41686, t41689, t41691, t41696)
}
