//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 908/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk908(t2365: f64, t35611: f64, t6111: f64, t36762: f64, t7785: f64, t44712: f64, t723: f64) -> (f64, f64, f64) {
    let t45414 = t6111 * t2365 * t35611;
    let t45415 = 0.59584149919750711116e-1_f64 * t45414;
    let t45421 = t36762 * t7785;
    let t45423 = t44712 * t723;
    (t45415, t45421, t45423)
}
