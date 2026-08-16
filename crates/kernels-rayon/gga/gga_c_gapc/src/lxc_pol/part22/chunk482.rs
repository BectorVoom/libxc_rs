//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 482/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk482(t2315: f64, t966: f64, t298: f64, t181: f64, t2250: f64, t268: f64) -> (f64, f64, f64) {
    let t2669 = t966 * t2315;
    let t2670 = t298 * t2669;
    let t2671 = t181 * t2670;
    let t2674 = t268 * t2250;
    (t2669, t2671, t2674)
}
