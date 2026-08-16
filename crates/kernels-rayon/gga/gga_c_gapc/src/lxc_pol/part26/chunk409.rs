//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 409/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk409(t2101: f64, t712: f64, t704: f64, t233: f64, t241: f64, t2091: f64, t374: f64, t78: f64, t1224: f64, t46: f64, t1225: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2102 = t2101 * t712;
    let t2105 = t704 * t704;
    let t2106 = 1.0_f64 / t2105;
    let t2107 = t233 * t2106;
    let t2108 = t241 * t241;
    let t2109 = 1.0_f64 / t2108;
    let t2110 = t2091 * t2109;
    let t2116 = t78 * t374;
    let t2120 = t46 * t1224;
    let t2121 = t1225 * t381;
    (t2102, t2107, t2110, t2116, t2120, t2121)
}
