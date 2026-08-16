//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 19/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk19(t11: f64, t14: f64, t17: f64, t25: f64) -> (f64, f64, f64) {
    let t51 = 0.51785e1_f64 * t14 + 0.905775e0_f64 * t11 + 0.1100325e0_f64 * t17 + 0.1241775e0_f64 * t25;
    let t54 = 1.0_f64 + 0.29608574643216675549e2_f64 / t51;
    let t55 = f64::ln(t54);
    (t51, t54, t55)
}
