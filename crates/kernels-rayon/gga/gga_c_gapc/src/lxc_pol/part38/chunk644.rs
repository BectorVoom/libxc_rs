//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 644/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk644(t115: f64, t4048: f64, t122: f64, t125: f64, t1899: f64, t172: f64, t1908: f64) -> (f64, f64, f64, f64) {
    let t4049 = t115 * t4048;
    let t4050 = t4049 * t122;
    let t4052 = t125 * t1899;
    let t4054 = t1908 * t172;
    (t4049, t4050, t4052, t4054)
}
