//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 723/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk723(t190: f64, t200: f64, t1954: f64, t8489: f64, t1404: f64, t493: f64, t2928: f64, t2912: f64, t8459: f64, t2929: f64, t2941: f64, t1845: f64, t515: f64) -> (f64, f64, f64, f64, f64) {
    let t8534 = t190 * t200;
    let t8535 = t8534 * t1954;
    let t8536 = t8489 * t8535;
    let t8538 = t493 * t1404;
    let t8539 = t2928 * t8538;
    let t8541 = t8459 * t2912;
    let t8543 = t2941 * t2929;
    let t8545 = t1845 * t515;
    (t8536, t8539, t8541, t8543, t8545)
}
